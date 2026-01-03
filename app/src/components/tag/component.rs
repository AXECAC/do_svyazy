use std::collections::HashSet;

use leptos::{IntoView, component, prelude::*};
use share::Tags;
use web_sys::MouseEvent;


#[component]
pub fn Tag(tag: Tags, selected_tags: RwSignal<HashSet<i32>>) -> impl IntoView {
    let is_checked = move || selected_tags.get().contains(&tag.id);

    let on_click = move |_: MouseEvent| {
        let mut set = selected_tags.get();
        if set.contains(&tag.id) {
            set.remove(&tag.id);
        } else {
            set.insert(tag.id);
        }
        selected_tags.set(set);
    };

    view! {
        <div style="">
            <label>
                {tag.name.clone()}
                <input type="checkbox" name="tags" prop:checked=is_checked() on:click=on_click />
            </label>
        </div>
    }
}
