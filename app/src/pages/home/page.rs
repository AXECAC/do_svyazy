use std::rc::Rc;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let navigate = use_navigate();
    let go_to_register = Rc::new(move |_| {
        navigate("/register", Default::default());
    });

    view! {
        {
            Button(ButtonProps {
                class_name: "".to_string(),
                children: Children::to_children(|| "Перейти к регистрации"),
                on_click: Some(go_to_register)
            })
        }
    }
}
