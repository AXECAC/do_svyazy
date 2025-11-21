use leptos::{prelude::*, IntoView};

#[component]
pub fn Input(
    class_name: String,
    name: String,
    placeholder: String,
    value: ReadSignal<String>,
    on_input: WriteSignal<String>,
) -> impl IntoView {
    let all_classes = format!("{} {}", class_name, "input");

    view! {
        <input
            class=all_classes
            name=name
            placeholder=placeholder
            bind:value=(value, on_input)
        />
    }
}
