use crate::storage::{get_next_todo_id, with_todo_store, with_todo_store_mut};
use crate::types::*;
use ic_cdk::api::time;

const MAX_TODO_TEXT_LENGTH: usize = 1000;
const MAX_PAGINATION_LIMIT: u64 = 100;

pub fn validate_todo_text(text: &str) -> Result<(), String> {
    if text.trim().is_empty() {
        return Err("Todo text cannot be empty".to_string());
    }
    if text.len() > MAX_TODO_TEXT_LENGTH {
        return Err(format!(
            "Todo text cannot exceed {} characters",
            MAX_TODO_TEXT_LENGTH
        ));
    }
    Ok(())
}

pub fn validate_pagination(pagination: &PaginationInput) -> Result<(), String> {
    if pagination.limit == 0 {
        return Err("Limit must be greater than 0".to_string());
    }
    if pagination.limit > MAX_PAGINATION_LIMIT {
        return Err(format!("Limit cannot exceed {}", MAX_PAGINATION_LIMIT));
    }
    Ok(())
}

pub fn create_todo(input: CreateTodoInput) -> TodoResult {
    if let Err(err) = validate_todo_text(&input.text) {
        return TodoResult::Err(err);
    }

    let now = time();
    let id = get_next_todo_id();

    let todo = Todo {
        id,
        text: input.text.trim().to_string(),
        completed: false,
        created_at: now,
        updated_at: now,
    };

    with_todo_store_mut(|store| {
        store.insert(id, todo.clone());
    });

    TodoResult::Ok(todo)
}

pub fn get_todo_by_id(id: TodoId) -> TodoResult {
    with_todo_store(|store| match store.get(&id) {
        Some(todo) => TodoResult::Ok(todo),
        None => TodoResult::Err("Todo not found".to_string()),
    })
}

pub fn get_all_todos_paginated(pagination: PaginationInput) -> TodosResult {
    if let Err(err) = validate_pagination(&pagination) {
        return TodosResult::Err(err);
    }

    with_todo_store(|store| {
        let mut all_todos: Vec<Todo> = store.iter().map(|(_, todo)| todo).collect();
        all_todos.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let total_count = all_todos.len() as u64;
        let start_index = pagination.offset as usize;

        if start_index >= all_todos.len() {
            return TodosResult::Ok(TodosPage {
                todos: vec![],
                total_count,
                has_more: false,
            });
        }

        let end_index = std::cmp::min(start_index + pagination.limit as usize, all_todos.len());

        let paginated_todos = all_todos[start_index..end_index].to_vec();
        let has_more = end_index < all_todos.len();

        TodosResult::Ok(TodosPage {
            todos: paginated_todos,
            total_count,
            has_more,
        })
    })
}

pub fn update_todo(input: UpdateTodoInput) -> TodoResult {
    with_todo_store_mut(|store| match store.get(&input.id) {
        Some(mut todo) => {
            let mut updated = false;

            if let Some(text) = input.text {
                if let Err(err) = validate_todo_text(&text) {
                    return TodoResult::Err(err);
                }
                todo.text = text.trim().to_string();
                updated = true;
            }

            if let Some(completed) = input.completed {
                todo.completed = completed;
                updated = true;
            }

            if updated {
                todo.updated_at = time();
                store.insert(input.id, todo.clone());
            }

            TodoResult::Ok(todo)
        }
        None => TodoResult::Err("Todo not found".to_string()),
    })
}

pub fn delete_todo_by_id(id: TodoId) -> DeleteResult {
    with_todo_store_mut(|store| match store.remove(&id) {
        Some(_) => DeleteResult::Ok(true),
        None => DeleteResult::Err("Todo not found".to_string()),
    })
}

pub fn get_todo_count() -> u64 {
    with_todo_store(|store| store.len())
}
