use leptos::ev::*;
use leptos::*;
#[component]

pub fn text_editor(cx: Scope) -> impl IntoView {
    let text = create_signal(cx, String::new());

    let on_paste = move |e: ClipboardEvent| {
        if let Some(clipboard_data) = e.clipboard_data() {
            let pasted_text = clipboard_data.get_data("text/plain").unwrap_or_default();
            text.set(pasted_text);
        }
    };

    view! { cx,
        <div class="p-6 max-w-lg mx-auto bg-white rounded-xl shadow-md space-y-4">
            <textarea
                class="w-full h-64 p-4 font-mono border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value=text
                on:paste=on_paste
                placeholder="Write your notes here with Markdown support..."
            />
            <button
                class="px-4 py-2 bg-blue-500 text-white font-semibold rounded-md shadow hover:bg-blue-600"
                on:click=move |_| {
                    web_sys::window().unwrap().navigator().clipboard().unwrap().write_text(&*text.get()).unwrap();
                }
            >
                { "Copy to Clipboard" }
            </button>
        </div>
    }
}
