use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="app-container">
            <h1>{ "Developer-Friendly Note-Taking App" }</h1>
            <text_editor::TextEditor />
        </div>
    }
}
