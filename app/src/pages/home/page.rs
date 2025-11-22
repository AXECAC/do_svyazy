use leptos::prelude::*;
use leptos::server::codee::string::FromToStringCodec;
use leptos_router::hooks::use_navigate;
use leptos_use::use_cookie;

/// Renders the home page of your application.

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            {
                move || {
                    let navigate_to_home = use_navigate();
                    let navigate_to_login = use_navigate();

                    Effect::new(move |_| {
                        let (token, _set_tocken) = use_cookie::<String, FromToStringCodec>("auth_token");

                        if let Some(tok) = token.get() {
                            if !tok.is_empty() {
                                navigate_to_login("/friends", Default::default())
                            } else {
                                navigate_to_home("/register", Default::default())
                            }
                        } else {
                            navigate_to_home("/register", Default::default())
                        }
                    });
                }
            }
        </div>
    }
}
