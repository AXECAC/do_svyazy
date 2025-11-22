use leptos::{html::*, prelude::*};

use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::pages::{friends::page::FriendsPage, home::page::HomePage, login::page::LoginPage, register::page::RegisterPage, tags::page::TagsPage};

mod components;
mod pages;
mod api;


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/do_svyazy.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("login") view=LoginPage/>
                    <Route path=StaticSegment("register") view=RegisterPage/>
                    <Route path=StaticSegment("tags") view=TagsPage/>
                    <Route path=StaticSegment("friends") view=FriendsPage/>
                </Routes>
            </main>
        </Router>
    }
}

