mod app;
mod markdown_support;
mod seed_phrase_sync;
mod text_editor;

use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <link rel="stylesheet" href="assets/styles/main.css" />
            <app::App />
        }
    });
}
