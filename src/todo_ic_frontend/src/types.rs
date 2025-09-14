use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CreateTodoInput {
    pub text: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct UpdateTodoInput {
    pub id: u64,
    pub text: Option<String>,
    pub completed: Option<bool>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct PaginationInput {
    pub offset: u64,
    pub limit: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq)]
pub struct TodosPage {
    pub todos: Vec<Todo>,
    pub total_count: u64,
    pub has_more: bool,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum TodoResult {
    Ok(Todo),
    Err(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum TodosResult {
    Ok(TodosPage),
    Err(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum DeleteResult {
    Ok(bool),
    Err(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Network {
    Local,
    Testnet,
    Mainnet,
}

impl Network {
    pub fn get_url(&self) -> &'static str {
        match self {
            Network::Local => "http://127.0.0.1:4943",
            Network::Testnet => "https://ic0.app",
            Network::Mainnet => "https://ic0.app",
        }
    }

    pub fn get_canister_id(&self) -> &'static str {
        match self {
            Network::Local => "rrkah-fqaaa-aaaaa-aaaaq-cai", // Default local canister ID
            Network::Testnet => "25x2w-paaaa-aaaab-qackq-cai",
            Network::Mainnet => "vyggl-hpuoy-22jqe-6povn-6glf6-z2rle-wgphx-nyv6a-xbomq-e6knd-iae", // Replace when deployed
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Network::Local => "Local",
            Network::Testnet => "Testnet",
            Network::Mainnet => "Mainnet",
        }
    }
}