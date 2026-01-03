use http::StatusCode;
use leptos::{config::ConfFile, prelude::*, server::codee::string::FromToStringCodec};
use leptos_router::hooks::use_navigate;
use leptos_use::{use_cookie_with_options, UseCookieOptions};
use share::LoginUser;

pub async fn handle_login(conf: ReadSignal<ConfFile>, user: LoginUser, error: WriteSignal<String>) {
    let response = reqwest::Client::new()
        .post(format!(
            "http://{}/login",
            conf.get().leptos_options.site_addr
        ))
        .json(&user)
        .send()
        .await;
    match response {
        Ok(resp) => {
            match resp.status() {
                StatusCode::OK => {
                    if let Ok(token) = resp.json::<String>().await {
                        let (_token, set_token) =
                            use_cookie_with_options::<String, FromToStringCodec>(
                                "auth_token",
                                UseCookieOptions::default().secure(false).path("/"),
                            );
                        Effect::new(move |_| {
                            set_token.set(Some(token.clone()));
                        });
                        // Переходим на домашнюю страницу
                        let navigate = use_navigate();
                        navigate("/friends", Default::default());
                    }
                }
                StatusCode::NOT_FOUND => {
                    error.set("Не правильная почта или пароль".to_string());
                }
                _ => {
                    error.set("Сервер помер".to_string());
                }
            }
        }
        Err(_) => {
            error.set("Помянем сеть".to_string());
        }
    }
}
