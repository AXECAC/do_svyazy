use leptos::prelude::*;


#[component]
pub fn FriendsPage() -> impl IntoView {
    // Состояние для поискового запроса
    let (search_text, set_search_text) = signal(String::new());
    // Состояние для выбранной вкладки: "all", "friends", "recommended"
    let (selected_tab, set_selected_tab) = signal("all".to_string());

    let on_search_click = move |_| {
    };

    view! {
        <div>
            // Поисковая строка
            <input
                type="text"
                placeholder="Поиск по имени..."
                prop:value=search_text.get()
                on:input=move |ev| {
                    let input = event_target_value(&ev);
                    set_search_text.set(input);
                }
            />
            <button on:click=on_search_click>"Поиск"</button>

            // Кнопки переключения фильтра
            <div style="display: flex; gap: 10px; margin-top: 16px;">
                <button
                    class:selected=move || selected_tab.get() == "all"
                    on:click=move |_| set_selected_tab.set("all".to_string())
                >"Все люди"</button>
                <button
                    class:selected=move || selected_tab.get() == "friends"
                    on:click=move |_| set_selected_tab.set("friends".to_string())
                >"Друзья"</button>
                <button
                    class:selected=move || selected_tab.get() == "recommended"
                    on:click=move |_| set_selected_tab.set("recommended".to_string())
                >"Рекомендации"</button>
            </div>

            // Пустой блок для вывода людей
            <div style="margin-top: 20px;">
                {}
            </div>
        </div>
    }
}

