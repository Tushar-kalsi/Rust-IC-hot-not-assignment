use leptos::*;
use crate::types::{Todo, Network};
use crate::ic_client::IcClient;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn TodoList(
    network: ReadSignal<Network>,
    refresh_trigger: ReadSignal<bool>,
) -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::<Todo>::new());
    let (is_loading, set_is_loading) = create_signal(true);
    let (error_message, set_error_message) = create_signal(None::<String>);
    let (total_count, set_total_count) = create_signal(0u64);

    let load_todos = move || {
        set_is_loading.set(true);
        set_error_message.set(None);

        let current_network = network.get();

        spawn_local(async move {
            match IcClient::new(current_network).await {
                Ok(client) => {
                    match client.get_all_todos(0, 50).await {
                        Ok(page) => {
                            set_todos.set(page.todos);
                            set_total_count.set(page.total_count);
                        }
                        Err(err) => {
                            set_error_message.set(Some(format!("Failed to load todos: {}", err)));
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

    // Load todos when component mounts or network changes
    create_effect(move |_| {
        network.track();
        load_todos();
    });

    // Reload todos when refresh is triggered
    create_effect(move |_| {
        refresh_trigger.track();
        load_todos();
    });

    let toggle_todo = move |todo: Todo| {
        let current_network = network.get();

        spawn_local(async move {
            match IcClient::new(current_network).await {
                Ok(client) => {
                    match client.update_todo_completed(todo.id, !todo.completed).await {
                        Ok(_) => {
                            load_todos();
                        }
                        Err(err) => {
                            set_error_message.set(Some(format!("Failed to update todo: {}", err)));
                        }
                    }
                }
                Err(err) => {
                    set_error_message.set(Some(format!("Connection failed: {}", err)));
                }
            }
        });
    };

    let delete_todo = move |todo_id: u64| {
        let current_network = network.get();

        spawn_local(async move {
            match IcClient::new(current_network).await {
                Ok(client) => {
                    match client.delete_todo(todo_id).await {
                        Ok(_) => {
                            load_todos();
                        }
                        Err(err) => {
                            set_error_message.set(Some(format!("Failed to delete todo: {}", err)));
                        }
                    }
                }
                Err(err) => {
                    set_error_message.set(Some(format!("Connection failed: {}", err)));
                }
            }
        });
    };

    view! {
        <div class="todo-list">
            <div class="todo-header">
                <h2>"📝 Todo List"</h2>
                <div class="todo-stats">
                    <span class="todo-count">
                        {move || format!("Total: {}", total_count.get())}
                    </span>
                    <span class="todo-completed">
                        {move || {
                            let completed = todos.get().iter().filter(|t| t.completed).count();
                            format!("Completed: {}", completed)
                        }}
                    </span>
                </div>
            </div>

            {move || error_message.get().map(|msg| view! {
                <div class="error-message">
                    "❌ " {msg}
                </div>
            })}

            {move || if is_loading.get() {
                view! {
                    <div class="loading">
                        "🔄 Loading todos..."
                    </div>
                }.into_view()
            } else if todos.get().is_empty() {
                view! {
                    <div class="empty-state">
                        <div class="empty-icon">"📭"</div>
                        <p>"No todos yet! Add your first todo above."</p>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="todos">
                        <For
                            each=move || todos.get()
                            key=|todo| todo.id
                            children=move |todo| {
                                let todo_for_toggle = todo.clone();
                                let todo_id = todo.id;

                                view! {
                                    <div class=format!(
                                        "todo-item {}",
                                        if todo.completed { "completed" } else { "" }
                                    )>
                                        <div class="todo-content">
                                            <button
                                                class="toggle-btn"
                                                on:click=move |_| toggle_todo(todo_for_toggle.clone())
                                            >
                                                {if todo.completed { "✅" } else { "⭕" }}
                                            </button>
                                            <span class="todo-text">
                                                {todo.text.clone()}
                                            </span>
                                            <span class="todo-id">
                                                {"#"}{todo.id}
                                            </span>
                                        </div>
                                        <button
                                            class="delete-btn"
                                            on:click=move |_| delete_todo(todo_id)
                                        >
                                            "🗑️"
                                        </button>
                                    </div>
                                }
                            }
                        />
                    </div>
                }.into_view()
            }}
        </div>
    }
}