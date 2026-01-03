use std::rc::Rc;

use leptos::{children::Children, ev::MouseEvent, html::ElementChild, prelude::*, IntoView};

stylance::import_crate_style!(style, "src/components/button/button.module.scss");

#[component]
pub fn Button(
    class_name: String,
    children: Children,
    #[prop(into, optional)] on_click: Option<Rc<dyn Fn(MouseEvent)>>,
) -> impl IntoView {
    let click_handler = {
        move |event: MouseEvent| {
            if let Some(some_on_click) = &on_click {
                some_on_click(event)
            }
        }
    };

    view! {
        <button class=format!("{} {}", class_name, style::button) on:click=click_handler>
            {children()}
        </button>
    }
}
