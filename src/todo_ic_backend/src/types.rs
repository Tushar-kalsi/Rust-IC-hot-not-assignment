use candid::{CandidType, Deserialize};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Serialize;
use std::borrow::Cow;

pub type TodoId = u64;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Todo {
    pub id: TodoId,
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
    pub id: TodoId,
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

impl Storable for Todo {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1024,
        is_fixed_size: false,
    };
}
