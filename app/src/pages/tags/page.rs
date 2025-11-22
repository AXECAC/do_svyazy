use std::rc::Rc;

use http::StatusCode;
use leptos::ev::MouseEvent;
use leptos::server::codee::string::FromToStringCodec;
use leptos::{html::*, prelude::*};
use leptos_router::hooks::use_navigate;
use leptos_use::{use_cookie_with_options, UseCookieOptions};

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;


#[component]
pub fn TagsPage() -> impl IntoView {
    view! {
        <div class="register_container">
            <h1 class="register_header">"Выберете ваши интересы"</h1>
            <p class="text_error">{}</p>
            {
                Button(ButtonProps {
                    class_name: "register_button".to_string(),
                    children: Children::to_children(|| "войти"),
                    on_click: None,
                })
            }

        </div>
    }
}
