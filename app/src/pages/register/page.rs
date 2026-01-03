use std::rc::Rc;
use leptos::ev::MouseEvent;
use leptos::{html::*, prelude::*};
use leptos_router::hooks::use_navigate;
use share::RegisterUser;

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;
use crate::components::input::component::{Input, InputProps};
use crate::pages::register::handlers::handle_register;

stylance::import_crate_style!(style, "src/pages/register/register.module.scss");

fn navigation() -> Rc<impl Fn(MouseEvent)> {
    let navigate_to_login = use_navigate();

    Rc::new(move |_| {
        navigate_to_login("/login", Default::default());
    })
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (conf, _set_conf) = signal(use_context::<ConfFile>().unwrap());
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (phio, set_phio) = signal("".to_string());

    let go_to_login = navigation();

    let (error, set_error) = signal(String::new());
    let input_class = style::register_input.to_string();

    let on_register_click = Rc::new({
        move |_| {
            let user = RegisterUser {
                username: phio.get(),
                email: email.get(),
                password: password.get(),
            };
            // Запускаем async задачу внутри Leptos
            wasm_bindgen_futures::spawn_local(async move {
                handle_register(conf, user, set_error).await;
            });
        }
    });

    view! {
        <div class=style::register_container>
            <h1 class=style::register_header>"Регистрация"</h1>
            <p class="text_error">{error}</p>
            {Input(InputProps {
                class_name: input_class.clone(),
                name: "register-name".to_string(),
                placeholder: "ФИО:".to_string(),
                type_: "text".to_string(),
                value: phio,
                on_input: set_phio,
            })}
            {Input(InputProps {
                class_name: input_class.clone(),
                name: "register-email".to_string(),
                placeholder: "Почта:".to_string(),
                type_: "email".to_string(),
                value: email,
                on_input: set_email,
            })}
            {Input(InputProps {
                class_name: input_class,
                name: "register-password".to_string(),
                placeholder: "Пароль:".to_string(),
                type_: "password".to_string(),
                value: password,
                on_input: set_password,
            })}
            {Button(ButtonProps {
                class_name: style::register_button.to_string(),
                children: Children::to_children(|| "Зарегестрироваться"),
                on_click: Some(on_register_click),
            })}

            <div class=style::go_to_container>
                {Button(ButtonProps {
                    class_name: style::go_to_button.to_string(),
                    children: Children::to_children(|| "Войти"),
                    on_click: Some(go_to_login),
                })}
            </div>
        </div>
    }
}
