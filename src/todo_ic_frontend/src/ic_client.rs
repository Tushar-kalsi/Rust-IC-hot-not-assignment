use crate::types::*;
use ic_agent::{Agent, Identity};
use candid::{encode_args, decode_args, Principal};
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use leptos::*;

pub struct IcClient {
    agent: Agent,
    canister_id: Principal,
    network: Network,
}

impl IcClient {
    pub async fn new(network: Network) -> Result<Self, String> {
        let url = network.get_url();
        let canister_id_str = network.get_canister_id();

        console::log_1(&format!("Connecting to {} at {}", network.display_name(), url).into());

        let agent = Agent::builder()
            .with_url(url)
            .build()
            .map_err(|e| format!("Failed to create agent: {:?}", e))?;

        if network == Network::Local {
            agent
                .fetch_root_key()
                .await
                .map_err(|e| format!("Failed to fetch root key: {:?}", e))?;
        }

        let canister_id = Principal::from_text(canister_id_str)
            .map_err(|e| format!("Invalid canister ID: {:?}", e))?;

        Ok(Self {
            agent,
            canister_id,
            network,
        })
    }

    pub async fn add_todo(&self, text: String) -> Result<Todo, String> {
        let input = CreateTodoInput { text };
        let args = encode_args((input,)).map_err(|e| format!("Encoding error: {:?}", e))?;

        let response = self.agent
            .update(&self.canister_id, "add_todo")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;

        let (result,): (TodoResult,) = decode_args(&response)
            .map_err(|e| format!("Decoding error: {:?}", e))?;

        match result {
            TodoResult::Ok(todo) => Ok(todo),
            TodoResult::Err(err) => Err(err),
        }
    }

    pub async fn get_all_todos(&self, offset: u64, limit: u64) -> Result<TodosPage, String> {
        let pagination = PaginationInput { offset, limit };
        let args = encode_args((pagination,)).map_err(|e| format!("Encoding error: {:?}", e))?;

        let response = self.agent
            .query(&self.canister_id, "get_all_todos")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| format!("Query failed: {:?}", e))?;

        let (result,): (TodosResult,) = decode_args(&response)
            .map_err(|e| format!("Decoding error: {:?}", e))?;

        match result {
            TodosResult::Ok(page) => Ok(page),
            TodosResult::Err(err) => Err(err),
        }
    }

    pub async fn update_todo_completed(&self, id: u64, completed: bool) -> Result<Todo, String> {
        let args = encode_args((id, completed)).map_err(|e| format!("Encoding error: {:?}", e))?;

        let response = self.agent
            .update(&self.canister_id, "update_todo_completed")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;

        let (result,): (TodoResult,) = decode_args(&response)
            .map_err(|e| format!("Decoding error: {:?}", e))?;

        match result {
            TodoResult::Ok(todo) => Ok(todo),
            TodoResult::Err(err) => Err(err),
        }
    }

    pub async fn delete_todo(&self, id: u64) -> Result<bool, String> {
        let args = encode_args((id,)).map_err(|e| format!("Encoding error: {:?}", e))?;

        let response = self.agent
            .update(&self.canister_id, "delete_todo")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;

        let (result,): (DeleteResult,) = decode_args(&response)
            .map_err(|e| format!("Decoding error: {:?}", e))?;

        match result {
            DeleteResult::Ok(success) => Ok(success),
            DeleteResult::Err(err) => Err(err),
        }
    }

    pub async fn get_todo_count(&self) -> Result<u64, String> {
        let response = self.agent
            .query(&self.canister_id, "get_todo_count")
            .with_arg(encode_args(()).unwrap())
            .call()
            .await
            .map_err(|e| format!("Query failed: {:?}", e))?;

        let (count,): (u64,) = decode_args(&response)
            .map_err(|e| format!("Decoding error: {:?}", e))?;

        Ok(count)
    }
}