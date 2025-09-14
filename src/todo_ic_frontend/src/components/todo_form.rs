use leptos::*;
use leptos::html::Input;
use crate::types::Network;
use crate::ic_client::IcClient;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn TodoForm(
    network: ReadSignal<Network>,
    on_todo_added: WriteSignal<bool>,
) -> impl IntoView {
    let (todo_text, set_todo_text) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(false);
    let (error_message, set_error_message) = create_signal(None::<String>);

    let input_ref = create_node_ref::<Input>();

    let add_todo = move |_| {
        let text = todo_text.get().trim().to_string();
        if text.is_empty() {
            set_error_message.set(Some("Please enter a todo item".to_string()));
            return;
        }

        set_is_loading.set(true);
        set_error_message.set(None);

        let current_network = network.get();

        spawn_local(async move {
            match IcClient::new(current_network).await {
                Ok(client) => {
                    match client.add_todo(text).await {
                        Ok(_) => {
                            set_todo_text.set(String::new());
                            on_todo_added.set(!on_todo_added.get()); // Trigger refresh
                            if let Some(input) = input_ref.get() {
                                let _ = input.focus();
                            }
                        }
                        Err(err) => {
                            set_error_message.set(Some(format!("Failed to add todo: {}", err)));
                        }
                    }
                }
                Err(err) => {
                    set_error_message.set(Some(format!("Connection failed: {}", err)));
                }
            }
            set_is_loading.set(false);
        });
    };

    let on_input = move |ev| {
        set_todo_text.set(event_target_value(&ev));
        set_error_message.set(None);
    };

    let on_keypress = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !is_loading.get() {
            add_todo(ev.into());
        }
    };

    view! {
        <div class="todo-form">
            <h2>"✨ Add New Todo"</h2>
            <div class="form-group">
                <input
                    type="text"
                    placeholder="What needs to be done?"
                    class="todo-input"
                    prop:value=move || todo_text.get()
                    on:input=on_input
                    on:keypress=on_keypress
                    prop:disabled=move || is_loading.get()
                    node_ref=input_ref
                />
                <button
                    class="add-btn"
                    on:click=add_todo
                    prop:disabled=move || is_loading.get() || todo_text.get().trim().is_empty()
                >
                    {move || if is_loading.get() { "➕ Adding..." } else { "➕ Add Todo" }}
                </button>
            </div>
            {move || error_message.get().map(|msg| view! {
                <div class="error-message">
                    "❌ " {msg}
                </div>
            })}
        </div>
    }
}