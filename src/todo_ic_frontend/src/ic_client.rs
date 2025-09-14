use crate::types::*;
use web_sys::console;
use leptos::*;
use web_sys::{window, Storage};
use serde_json;

pub struct IcClient {
    network: Network,
    storage: Storage,
}

impl IcClient {
    pub async fn new(network: Network) -> Result<Self, String> {
        console::log_1(&format!("Connecting to {} (using local storage mock)", network.display_name()).into());

        let window = window().ok_or("No window object")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to get local storage")?
            .ok_or("Local storage not available")?;

        Ok(Self {
            network,
            storage,
        })
    }

    fn get_todos_key(&self) -> String {
        format!("todos_{}", self.network.display_name().to_lowercase())
    }

    fn get_next_id(&self) -> u64 {
        let key = format!("next_id_{}", self.network.display_name().to_lowercase());
        let current_id = self.storage.get_item(&key)
            .unwrap_or(None)
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(1);
        
        let next_id = current_id + 1;
        let _ = self.storage.set_item(&key, &next_id.to_string());
        current_id
    }

    fn get_all_todos_from_storage(&self) -> Vec<Todo> {
        let key = self.get_todos_key();
        self.storage.get_item(&key)
            .unwrap_or(None)
            .and_then(|json_str| serde_json::from_str(&json_str).ok())
            .unwrap_or_default()
    }

    fn save_todos_to_storage(&self, todos: &[Todo]) -> Result<(), String> {
        let key = self.get_todos_key();
        let json_str = serde_json::to_string(todos)
            .map_err(|e| format!("Failed to serialize todos: {:?}", e))?;
        
        self.storage.set_item(&key, &json_str)
            .map_err(|_| "Failed to save to local storage".to_string())
    }

    pub async fn add_todo(&self, text: String) -> Result<Todo, String> {
        let mut todos = self.get_all_todos_from_storage();
        let now = js_sys::Date::now() as u64;
        
        let new_todo = Todo {
            id: self.get_next_id(),
            text,
            completed: false,
            created_at: now,
            updated_at: now,
        };
        
        todos.push(new_todo.clone());
        self.save_todos_to_storage(&todos)?;
        
        Ok(new_todo)
    }

    pub async fn get_all_todos(&self, offset: u64, limit: u64) -> Result<TodosPage, String> {
        let todos = self.get_all_todos_from_storage();
        let total_count = todos.len() as u64;
        
        let start = offset as usize;
        let end = std::cmp::min(start + limit as usize, todos.len());
        
        let page_todos = if start < todos.len() {
            todos[start..end].to_vec()
        } else {
            Vec::new()
        };
        
        let has_more = end < todos.len();
        
        Ok(TodosPage {
            todos: page_todos,
            total_count,
            has_more,
        })
    }

    pub async fn update_todo_completed(&self, id: u64, completed: bool) -> Result<Todo, String> {
        let mut todos = self.get_all_todos_from_storage();
        
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            todo.completed = completed;
            todo.updated_at = js_sys::Date::now() as u64;
            let updated_todo = todo.clone();
            self.save_todos_to_storage(&todos)?;
            Ok(updated_todo)
        } else {
            Err("Todo not found".to_string())
        }
    }

    pub async fn delete_todo(&self, id: u64) -> Result<bool, String> {
        let mut todos = self.get_all_todos_from_storage();
        let initial_len = todos.len();
        
        todos.retain(|t| t.id != id);
        
        if todos.len() < initial_len {
            self.save_todos_to_storage(&todos)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_todo_count(&self) -> Result<u64, String> {
        let todos = self.get_all_todos_from_storage();
        Ok(todos.len() as u64)
    }
}