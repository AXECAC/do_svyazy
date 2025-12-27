use leptos::{prelude::*, IntoView};

stylance::import_crate_style!(style, "src/components/input/input.module.scss");

#[component]
pub fn Input(
    class_name: String,
    name: String,
    placeholder: String,
    type_: String,
    value: ReadSignal<String>,
    on_input: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <input
            class=format!("{} {}", class_name, style::input)
            name=name
            placeholder=placeholder
            type=type_
            bind:value=(value, on_input)
        />
    }
}
