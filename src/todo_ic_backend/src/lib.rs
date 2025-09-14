mod service;
mod storage;
mod types;

use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use service::*;
use storage::init_storage;
use types::*;

#[init]
fn init() {
    init_storage();
}

#[pre_upgrade]
fn pre_upgrade() {}

#[post_upgrade]
fn post_upgrade() {}

#[update]
pub fn add_todo(input: CreateTodoInput) -> TodoResult {
    create_todo(input)
}

#[query]
fn get_todo(id: TodoId) -> TodoResult {
    get_todo_by_id(id)
}

#[query]
fn get_all_todos(pagination: PaginationInput) -> TodosResult {
    get_all_todos_paginated(pagination)
}

#[update]
fn update_todo_text(id: TodoId, text: String) -> TodoResult {
    let input = UpdateTodoInput {
        id,
        text: Some(text),
        completed: None,
    };
    service::update_todo(input)
}

#[update]
fn update_todo_completed(id: TodoId, completed: bool) -> TodoResult {
    let input = UpdateTodoInput {
        id,
        text: None,
        completed: Some(completed),
    };
    service::update_todo(input)
}

#[update]
fn delete_todo(id: TodoId) -> DeleteResult {
    delete_todo_by_id(id)
}

#[query]
fn get_todo_count() -> u64 {
    service::get_todo_count()
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
