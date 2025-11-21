use std::rc::Rc;

use leptos::ev::MouseEvent;
use leptos::{html::*, prelude::*};
use leptos_router::hooks::use_navigate;

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;
use crate::components::input::component::{Input, InputProps};

fn navigation() -> (Rc<impl Fn(MouseEvent)>, Rc<impl Fn(MouseEvent)>) {
    let navigate_to_home = use_navigate();
    let navigate_to_login = use_navigate();

    (
        Rc::new(move |_| {
            navigate_to_home("/", Default::default());
        }),
        Rc::new(move |_| {
            navigate_to_login("/login", Default::default());
        }),
    )
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (phio, set_phio) = signal("".to_string());

    let (go_home, go_to_login) = navigation();

    let input_class = "register_input".to_string();

    view! {
        <form class="register_container">
            <h1 class="register_header">"Добро пожаловать"</h1>
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
                    on_click: None,
                })
            }

            <div class="go_to_container">
                {
                    Button(ButtonProps {
                        class_name: "go_to_button".to_string(),
                        children: Children::to_children(|| "На главную"),
                        on_click: Some(go_home),
                    })
                }
                {
                    Button(ButtonProps {
                        class_name: "go_to_button".to_string(),
                        children: Children::to_children(|| "Войти"),
                        on_click: Some(go_to_login),
                    })
                }
            </div>
        </form>
    }
}
