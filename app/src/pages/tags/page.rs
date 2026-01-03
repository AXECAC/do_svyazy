use std::collections::HashSet;
use std::rc::Rc;
use leptos::ev::MouseEvent;
use leptos::server::codee::string::FromToStringCodec;
use leptos::{html::*, prelude::*};
use leptos_use::use_cookie;
use share::Tags;

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;
use crate::pages::tags::handlers::{handle_get_tags, handle_set_tags};

#[component]
fn Tag(tag: Tags, selected_tags: RwSignal<HashSet<i32>>) -> impl IntoView {
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

fn get_token(token_s: WriteSignal<String>) {
    Effect::new(move |_| {
        let (token, _set_token) = use_cookie::<String, FromToStringCodec>("auth_token");

        if let Some(tok) = token.get() {
            token_s.set(tok);
        } else {
            token_s.set(String::new());
        }
    });
}

#[component]
pub fn TagsPage() -> impl IntoView {
    let (conf, _set_conf) = signal(use_context::<ConfFile>().unwrap());
    let (error, set_error) = signal(String::new());
    let (tags, set_tags) = signal::<Vec<Tags>>(vec![]);
    let selected_tags: RwSignal<HashSet<i32>> = RwSignal::new(HashSet::new());

    let on_confirm = Rc::new(move |_| {
        let chosen_tags: Vec<i32> = selected_tags.get().iter().cloned().collect();
        let (token, set_token) = signal(String::new());
        get_token(set_token);
        wasm_bindgen_futures::spawn_local(async move {
            handle_set_tags(conf, chosen_tags, token.get(), set_error).await;
        });
    });

    view! {
        {move || {
            Effect::new(move || {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_tags = handle_get_tags(conf, set_error).await;
                    set_tags.set(fetched_tags);
                });
            });
        }}
        <div class="register_container">
            <h1 class="register_header">"Выберите ваши интересы"</h1>
            <p class="text_error">{error}</p>
            {move || {
                let items = tags.get();
                items
                    .iter()
                    .map(|item| {
                        view! {
                            <div>
                                <Tag tag=item.clone() selected_tags=selected_tags />
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
            {Button(ButtonProps {
                class_name: "register_button".to_string(),
                children: Children::to_children(|| "войти"),
                on_click: Some(on_confirm),
            })}
        </div>
    }
}
