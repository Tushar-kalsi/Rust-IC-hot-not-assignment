use leptos::*;
use crate::types::Network;

#[component]
pub fn NetworkSelector(
    current_network: ReadSignal<Network>,
    on_network_change: WriteSignal<Network>,
) -> impl IntoView {
    view! {
        <div class="network-selector">
            <h3>"🌐 Network Selection"</h3>
            <div class="network-buttons">
                <button
                    class=move || if current_network.get() == Network::Local {
                        "network-btn active local"
                    } else {
                        "network-btn local"
                    }
                    on:click=move |_| on_network_change.set(Network::Local)
                >
                    "🖥️ Local"
                </button>
                <button
                    class=move || if current_network.get() == Network::Testnet {
                        "network-btn active testnet"
                    } else {
                        "network-btn testnet"
                    }
                    on:click=move |_| on_network_change.set(Network::Testnet)
                >
                    "🧪 Testnet"
                </button>
                <button
                    class=move || if current_network.get() == Network::Mainnet {
                        "network-btn active mainnet"
                    } else {
                        "network-btn mainnet"
                    }
                    on:click=move |_| on_network_change.set(Network::Mainnet)
                >
                    "🌍 Mainnet"
                </button>
            </div>
            <div class="network-info">
                <p>
                    <strong>"Current: "</strong>
                    {move || current_network.get().display_name()}
                </p>
                <p class="network-id">
                    <strong>"Canister ID: "</strong>
                    <code>{move || current_network.get().get_canister_id()}</code>
                </p>
            </div>
        </div>
    }
}