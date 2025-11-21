use std::rc::Rc;

use leptos::prelude::*;
use leptos::server::codee::string::FromToStringCodec;
use leptos_router::hooks::use_navigate;
use leptos_use::use_cookie;

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let navigate = use_navigate();
    let go_to_register = Rc::new(move |_| {
        navigate("/register", Default::default());
    });

    let (token, _set_tocken) = use_cookie::<String, FromToStringCodec>("auth_token");
    view! {
        <div class="home_container">
            <p> {token.get()} </p>
            <img class="mascot" src="mascot.png"/>
            {
                Button(ButtonProps {
                    class_name: "".to_string(),
                    children: Children::to_children(|| "Создать аккаунт"),
                    on_click: Some(go_to_register)
                })
            }
        </div>
    }
}
