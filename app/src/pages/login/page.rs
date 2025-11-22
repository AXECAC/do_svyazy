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

fn navigation() -> Rc<impl Fn(MouseEvent)> {
    let navigate_to_login = use_navigate();

    Rc::new(move |_| {
        navigate_to_login("/register", Default::default());
    })
}

async fn handle_login(user: LoginUser, error: WriteSignal<String>) {
    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3000/login")
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
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());

    let go_to_register = navigation();

    let (error, set_error) = signal(String::new());
    let input_class = "register_input".to_string();
    // Обработчик клика на кнопку регистрации
    let on_login_click = Rc::new({
        let email = email.clone();
        let password = password.clone();

        move |_| {
            let user = LoginUser {
                email: email.get(),
                password: password.get(),
            };
            // Запускаем async задачу внутри Leptos
            wasm_bindgen_futures::spawn_local(async move {
                handle_login(user, set_error).await;
            });
        }
    });

    view! {
        <div class="register_container">
            <h1 class="register_header">"Логин"</h1>
            <p class="text_error">{error}</p>
            {
                Input(InputProps{
                    class_name: input_class.clone(),
                    name: "login-email".to_string(),
                    placeholder: "Почта:".to_string(),
                    type_: "email".to_string(),
                    value: email,
                    on_input: set_email
                })
            }
            {
                Input(InputProps{
                    class_name: input_class,
                    name: "login-password".to_string(),
                    placeholder: "Пароль:".to_string(),
                    type_: "password".to_string(),
                    value: password,
                    on_input: set_password
                })
            }
            {
                Button(ButtonProps {
                    class_name: "register_button".to_string(),
                    children: Children::to_children(|| "войти"),
                    on_click: Some(on_login_click),
                })
            }

            <div class="go_to_container">
                {
                    Button(ButtonProps {
                        class_name: "go_to_button".to_string(),
                        children: Children::to_children(|| "Зарегестрироваться"),
                        on_click: Some(go_to_register),
                    })
                }
            </div>
        </div>
    }
}
