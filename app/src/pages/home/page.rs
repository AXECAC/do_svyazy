use std::rc::Rc;

use leptos::prelude::*;
use leptos::server::codee::string::FromToStringCodec;
use leptos_router::hooks::use_navigate;
use leptos_use::use_cookie;

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;
/// Renders the home page of your application.

fn test() {
    let navigate_to_home = use_navigate();
    let navigate_to_login = use_navigate();

    Effect::new(move |_| {
        let (token, _set_tocken) = use_cookie::<String, FromToStringCodec>("auth_token");

        if let Some(tok) = token.get() {
            if !tok.is_empty() {
                navigate_to_login("/login", Default::default())
            } else {
                navigate_to_home("/register", Default::default())
            }
        } else {
            navigate_to_home("/register", Default::default())
        }
    });
}
#[component]
pub fn HomePage() -> impl IntoView {
    let nav = Rc::new({
        move |_| {
            test();
        }
    });

    view! {
        <div>
            {
                Button(ButtonProps {
                    class_name: "register_button".to_string(),
                    children: Children::to_children(|| "Зарегестрироваться"),
                    on_click: Some(nav),
                })
            }
            {
                move || {
                    test();
                }
            }
        </div>
    }
}
