use leptos::*;
use leptos_meta::*;
use crate::components::{TodoForm, TodoList, NetworkSelector};
use crate::types::Network;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (current_network, set_current_network) = create_signal(Network::Testnet);
    let (refresh_trigger, set_refresh_trigger) = create_signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/todo_ic_frontend.css"/>
        <Title text="Todo IC - Decentralized Todo App"/>
        <Meta name="description" content="A decentralized todo application built on the Internet Computer"/>

        <div class="app">
            <header class="app-header">
                <div class="header-content">
                    <h1 class="app-title">
                        <span class="gradient-text">"🚀 Todo IC"</span>
                    </h1>
                    <p class="app-subtitle">
                        "Decentralized Todo App on the Internet Computer"
                    </p>
                </div>
            </header>

            <main class="app-main">
                <div class="container">
                    <NetworkSelector
                        current_network=current_network
                        on_network_change=set_current_network
                    />

                    <TodoForm
                        network=current_network
                        on_todo_added=set_refresh_trigger
                    />

                    <TodoList
                        network=current_network
                        refresh_trigger=refresh_trigger
                    />
                </div>
            </main>

            <footer class="app-footer">
                <p>"Built with "
                    <span class="heart">"❤️"</span>
                    " using Leptos + Internet Computer"
                </p>
            </footer>
        </div>
    }
}