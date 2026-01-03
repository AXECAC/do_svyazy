use http::StatusCode;
use leptos::{config::ConfFile, prelude::*};
use leptos_router::hooks::use_navigate;
use share::Tags;


pub async fn handle_get_tags(conf: ReadSignal<ConfFile>, error: WriteSignal<String>) -> Vec<Tags> {
    let response = reqwest::Client::new()
        .get(format!(
            "http://{}/get_base_tags",
            conf.get().leptos_options.site_addr
        ))
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

pub async fn handle_set_tags(
    conf: ReadSignal<ConfFile>,
    tags: Vec<i32>,
    email: String,
    error: WriteSignal<String>,
) {
    let response = reqwest::Client::new()
        .post(format!(
            "http://{}/set_base_tags",
            conf.get().leptos_options.site_addr
        ))
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

