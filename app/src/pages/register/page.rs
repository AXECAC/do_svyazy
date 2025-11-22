use std::rc::Rc;

use http::StatusCode;
use leptos::ev::MouseEvent;
use leptos::server::codee::string::FromToStringCodec;
use leptos::{html::*, prelude::*};
use leptos_router::hooks::use_navigate;
use leptos_use::{use_cookie_with_options, UseCookieOptions};
use share::RegisterUser;

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;
use crate::components::input::component::{Input, InputProps};

fn navigation() -> Rc<impl Fn(MouseEvent)> {
    let navigate_to_login = use_navigate();

    Rc::new(move |_| {
        navigate_to_login("/login", Default::default());
    })
}

async fn handle_register(user: RegisterUser) {
    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3000/registration")
        .json(&user)
        .send()
        .await;
    web_sys::console::log_1(&"aboba".into());
    match response {
        Ok(resp) => {
            web_sys::console::log_1(&"Ок".into());
            match resp.status() {
                StatusCode::OK => {
                    web_sys::console::log_1(&"Not conflict".into());

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
                        navigate("/", Default::default());
                    }
                }
                StatusCode::CONFLICT => {
                    web_sys::console::log_1(&"aboba".into());
                }
                _ => {
                    web_sys::console::log_1(&"500".into());
                }
            }
        }
        Err(_) => {
            web_sys::console::log_1(&"Помянем сеть".into());
        }
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (phio, set_phio) = signal("".to_string());

    let go_to_login = navigation();

    let input_class = "register_input".to_string();
    // Обработчик клика на кнопку регистрации
    let on_register_click = Rc::new({
        let email = email.clone();
        let password = password.clone();
        let phio = phio.clone();

        move |_| {
            let user = RegisterUser {
                username: phio.get(),
                email: email.get(),
                password: password.get(),
            };
            // Запускаем async задачу внутри Leptos
            wasm_bindgen_futures::spawn_local(async move {
                handle_register(user).await;
            });
        }
    });

    view! {
        <div class="register_container">
            <h1 class="register_header">"Регистрация"</h1>
            {
                Input(InputProps{
                    class_name: input_class.clone(),
                    name: "register-email".to_string(),
                    placeholder: "ФИО:".to_string(),
                    type_: "text".to_string(),
                    value: phio,
                    on_input: set_phio
                })
            }
            {
                Input(InputProps{
                    class_name: input_class.clone(),
                    name: "register-email".to_string(),
                    placeholder: "Почта:".to_string(),
                    type_: "email".to_string(),
                    value: email,
                    on_input: set_email
                })
            }
            {
                Input(InputProps{
                    class_name: input_class,
                    name: "register-password".to_string(),
                    placeholder: "Пароль:".to_string(),
                    type_: "password".to_string(),
                    value: password,
                    on_input: set_password
                })
            }
            {
                Button(ButtonProps {
                    class_name: "register_button".to_string(),
                    children: Children::to_children(|| "Зарегестрироваться"),
                    on_click: Some(on_register_click),
                })
            }

            <div class="go_to_container">
                {
                    Button(ButtonProps {
                        class_name: "go_to_button".to_string(),
                        children: Children::to_children(|| "Войти"),
                        on_click: Some(go_to_login),
                    })
                }
            </div>
        </div>
    }
}
