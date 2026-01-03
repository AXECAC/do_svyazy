use std::rc::Rc;

use http::StatusCode;
use leptos::ev::MouseEvent;
use leptos::server::codee::string::FromToStringCodec;
use leptos::{html::*, prelude::*};
use leptos_router::hooks::use_navigate;
use leptos_use::{use_cookie_with_options, UseCookieOptions};
use share::LoginUser;

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;
use crate::components::input::component::{Input, InputProps};

stylance::import_crate_style!(style, "src/pages/login/login.module.scss");

fn navigation() -> Rc<impl Fn(MouseEvent)> {
    let navigate_to_login = use_navigate();

    Rc::new(move |_| {
        navigate_to_login("/register", Default::default());
    })
}

async fn handle_login(conf: ReadSignal<ConfFile>, user: LoginUser, error: WriteSignal<String>) {
    let response = reqwest::Client::new()
        .post(format!(
            "http://{}/login",
            conf.get().leptos_options.site_addr
        ))
        .json(&user)
        .send()
        .await;
    match response {
        Ok(resp) => {
            match resp.status() {
                StatusCode::OK => {
                    if let Ok(token) = resp.json::<String>().await {
                        let (_token, set_token) =
                            use_cookie_with_options::<String, FromToStringCodec>(
                                "auth_token",
                                UseCookieOptions::default().secure(false).path("/"),
                            );
                        Effect::new(move |_| {
                            set_token.set(Some(token.clone()));
                        });
                        // Переходим на домашнюю страницу
                        let navigate = use_navigate();
                        navigate("/friends", Default::default());
                    }
                }
                StatusCode::NOT_FOUND => {
                    error.set("Не правильная почта или пароль".to_string());
                }
                _ => {
                    error.set("Сервер помер".to_string());
                }
            }
        }
        Err(_) => {
            error.set("Помянем сеть".to_string());
        }
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let (conf, _set_conf) = signal(use_context::<ConfFile>().unwrap());
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());

    let go_to_register = navigation();

    let (error, set_error) = signal(String::new());
    let input_class = style::login_input.to_string();

    let on_login_click = Rc::new({
        move |_| {
            let user = LoginUser {
                email: email.get(),
                password: password.get(),
            };
            // Запускаем async задачу внутри Leptos
            wasm_bindgen_futures::spawn_local(async move {
                handle_login(conf, user, set_error).await;
            });
        }
    });

    view! {
        <div class=style::login_container.to_string()>
            <h1 class=style::login_header.to_string()>"Вход"</h1>
            <p class="text_error">{error}</p>
            {Input(InputProps {
                class_name: input_class.clone(),
                name: "login-email".to_string(),
                placeholder: "Почта:".to_string(),
                type_: "email".to_string(),
                value: email,
                on_input: set_email,
            })}
            {Input(InputProps {
                class_name: input_class,
                name: "login-password".to_string(),
                placeholder: "Пароль:".to_string(),
                type_: "password".to_string(),
                value: password,
                on_input: set_password,
            })}
            {Button(ButtonProps {
                class_name: style::login_button.to_string(),
                children: Children::to_children(|| "войти"),
                on_click: Some(on_login_click),
            })}

            <div class=style::go_to_container>
                {Button(ButtonProps {
                    class_name: style::go_to_button.to_string(),
                    children: Children::to_children(|| "Зарегестрироваться"),
                    on_click: Some(go_to_register),
                })}
            </div>
        </div>
    }
}
