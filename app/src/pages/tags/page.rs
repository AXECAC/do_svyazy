use std::collections::HashSet;
use std::rc::Rc;

use http::StatusCode;
use leptos::ev::MouseEvent;
use leptos::server::codee::string::FromToStringCodec;
use leptos::{html::*, prelude::*};
use leptos_router::hooks::use_navigate;
use leptos_use::use_cookie;
use share::Tags;

use crate::components::button::component::Button;
use crate::components::button::component::ButtonProps;

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

async fn handle_get_tags(error: WriteSignal<String>) -> Vec<Tags> {
    let response = reqwest::Client::new()
        .get("http://127.0.0.1:3000/get_base_tags")
        .send()
        .await;
    match response {
        Ok(resp) => match resp.status() {
            StatusCode::OK => match resp.json::<Vec<Tags>>().await {
                Ok(tags) => tags,
                Err(_) => {
                    error.set("Не удалось распарсить ответ с тегами".to_string());
                    vec![]
                }
            },
            _ => {
                error.set("Ошибка сервера при получении тегов".to_string());
                vec![]
            }
        },
        Err(_) => {
            error.set("Проблема с сетью при получении тегов".to_string());
            vec![]
        }
    }
}

async fn handle_set_tags(tags: Vec<i32>, email: String, error: WriteSignal<String>) {
    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3000/set_base_tags")
        .json(&(tags, email))
        .send()
        .await;
    match response {
        Ok(resp) => match resp.status() {
            StatusCode::OK => {
                let navigate = use_navigate();
                navigate("/friends", Default::default());
            }
            StatusCode::NOT_FOUND => error.set("Не найден ваш аккаунт".to_string()),
            _ => error.set("Ошибка сервера при получении тегов".to_string()),
        },
        Err(_) => error.set("Проблема с сетью при получении тегов".to_string()),
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
    // Signal хранит выбранный тег по id
    let (error, set_error) = signal(String::new());
    let (tags, set_tags) = signal::<Vec<Tags>>(vec![]);
    let selected_tags: RwSignal<HashSet<i32>> = RwSignal::new(HashSet::new());

    let on_confirm = Rc::new(move |_| {
        let chosen_tags: Vec<i32> = selected_tags.get().iter().cloned().collect();
        let (token, set_token) = signal(String::new());
        get_token(set_token);
        wasm_bindgen_futures::spawn_local(async move {
            handle_set_tags(chosen_tags, token.get(), set_error).await;
        });
    });

    view! {
        {move || {
            Effect::new(move || {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_tags = handle_get_tags(set_error).await;
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
