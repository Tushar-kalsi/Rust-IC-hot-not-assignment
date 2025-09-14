use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug)]
pub struct CreateTodoInput {
    pub text: String,
}

#[derive(Debug)]
pub struct UpdateTodoInput {
    pub id: u64,
    pub text: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug)]
pub struct PaginationInput {
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, PartialEq)]
pub struct TodosPage {
    pub todos: Vec<Todo>,
    pub total_count: u64,
    pub has_more: bool,
}

#[derive(Debug, PartialEq)]
pub enum TodoResult {
    Ok(Todo),
    Err(String),
}

#[derive(Debug, PartialEq)]
pub enum TodosResult {
    Ok(TodosPage),
    Err(String),
}

#[derive(Debug, PartialEq)]
pub enum DeleteResult {
    Ok(bool),
    Err(String),
}

pub struct TodoService {
    todos: HashMap<u64, Todo>,
    next_id: u64,
}

impl Default for TodoService {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoService {
    pub fn new() -> Self {
        Self {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    fn get_current_time() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    fn validate_todo_text(text: &str) -> Result<(), String> {
        if text.trim().is_empty() {
            return Err("Todo text cannot be empty".to_string());
        }
        if text.len() > 1000 {
            return Err("Todo text cannot exceed 1000 characters".to_string());
        }
        Ok(())
    }

    fn validate_pagination(pagination: &PaginationInput) -> Result<(), String> {
        if pagination.limit == 0 {
            return Err("Limit must be greater than 0".to_string());
        }
        if pagination.limit > 100 {
            return Err("Limit cannot exceed 100".to_string());
        }
        Ok(())
    }

    pub fn add_todo(&mut self, input: CreateTodoInput) -> TodoResult {
        if let Err(err) = Self::validate_todo_text(&input.text) {
            return TodoResult::Err(err);
        }

        let now = Self::get_current_time();
        let id = self.next_id;
        self.next_id += 1;

        let todo = Todo {
            id,
            text: input.text.trim().to_string(),
            completed: false,
            created_at: now,
            updated_at: now,
        };

        self.todos.insert(id, todo.clone());
        TodoResult::Ok(todo)
    }

    pub fn get_todo(&self, id: u64) -> TodoResult {
        match self.todos.get(&id) {
            Some(todo) => TodoResult::Ok(todo.clone()),
            None => TodoResult::Err("Todo not found".to_string()),
        }
    }

    pub fn get_all_todos(&self, pagination: PaginationInput) -> TodosResult {
        if let Err(err) = Self::validate_pagination(&pagination) {
            return TodosResult::Err(err);
        }

        let mut all_todos: Vec<Todo> = self.todos.values().cloned().collect();
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
    }

    pub fn update_todo(&mut self, input: UpdateTodoInput) -> TodoResult {
        match self.todos.get_mut(&input.id) {
            Some(todo) => {
                let mut updated = false;

                if let Some(text) = input.text {
                    if let Err(err) = Self::validate_todo_text(&text) {
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
                    todo.updated_at = Self::get_current_time();
                }

                TodoResult::Ok(todo.clone())
            }
            None => TodoResult::Err("Todo not found".to_string()),
        }
    }

    pub fn delete_todo(&mut self, id: u64) -> DeleteResult {
        match self.todos.remove(&id) {
            Some(_) => DeleteResult::Ok(true),
            None => DeleteResult::Err("Todo not found".to_string()),
        }
    }

    pub fn get_todo_count(&self) -> u64 {
        self.todos.len() as u64
    }
}

fn create_test_service() -> TodoService {
    TodoService::new()
}

fn create_sample_todo(service: &mut TodoService, text: &str) -> Todo {
    let input = CreateTodoInput {
        text: text.to_string(),
    };
    match service.add_todo(input) {
        TodoResult::Ok(todo) => todo,
        TodoResult::Err(e) => panic!("Failed to create todo: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo_success() {
        let mut service = create_test_service();
        let input = CreateTodoInput {
            text: "Learn Rust".to_string(),
        };

        let result = service.add_todo(input);

        match result {
            TodoResult::Ok(todo) => {
                assert_eq!(todo.id, 1);
                assert_eq!(todo.text, "Learn Rust");
                assert!(!todo.completed);
                assert!(todo.created_at > 0);
                assert_eq!(todo.created_at, todo.updated_at);
            }
            TodoResult::Err(e) => panic!("Expected success, got error: {}", e),
        }
    }

    #[test]
    fn test_add_todo_empty_text() {
        let mut service = create_test_service();
        let input = CreateTodoInput {
            text: "".to_string(),
        };

        let result = service.add_todo(input);

        match result {
            TodoResult::Ok(_) => panic!("Expected error for empty text"),
            TodoResult::Err(e) => assert_eq!(e, "Todo text cannot be empty"),
        }
    }

    #[test]
    fn test_get_todo_success() {
        let mut service = create_test_service();
        let created_todo = create_sample_todo(&mut service, "Test todo");

        let result = service.get_todo(created_todo.id);

        match result {
            TodoResult::Ok(todo) => {
                assert_eq!(todo.id, created_todo.id);
                assert_eq!(todo.text, created_todo.text);
            }
            TodoResult::Err(e) => panic!("Expected success, got error: {}", e),
        }
    }

    #[test]
    fn test_get_todo_not_found() {
        let service = create_test_service();

        let result = service.get_todo(999);

        match result {
            TodoResult::Ok(_) => panic!("Expected error for non-existent todo"),
            TodoResult::Err(e) => assert_eq!(e, "Todo not found"),
        }
    }

    #[test]
    fn test_get_all_todos_empty() {
        let service = create_test_service();
        let pagination = PaginationInput {
            offset: 0,
            limit: 10,
        };

        let result = service.get_all_todos(pagination);

        match result {
            TodosResult::Ok(page) => {
                assert_eq!(page.todos.len(), 0);
                assert_eq!(page.total_count, 0);
                assert!(!page.has_more);
            }
            TodosResult::Err(e) => panic!("Expected success, got error: {}", e),
        }
    }

    #[test]
    fn test_pagination_limit_zero() {
        let service = create_test_service();
        let pagination = PaginationInput {
            offset: 0,
            limit: 0,
        };

        let result = service.get_all_todos(pagination);

        match result {
            TodosResult::Ok(_) => panic!("Expected error for limit 0"),
            TodosResult::Err(e) => assert_eq!(e, "Limit must be greater than 0"),
        }
    }

    #[test]
    fn test_update_todo_text() {
        let mut service = create_test_service();
        let created_todo = create_sample_todo(&mut service, "Original text");

        let input = UpdateTodoInput {
            id: created_todo.id,
            text: Some("Updated text".to_string()),
            completed: None,
        };

        let result = service.update_todo(input);

        match result {
            TodoResult::Ok(todo) => {
                assert_eq!(todo.text, "Updated text");
                assert!(todo.updated_at >= created_todo.updated_at);
            }
            TodoResult::Err(e) => panic!("Expected success, got error: {}", e),
        }
    }

    #[test]
    fn test_delete_todo_success() {
        let mut service = create_test_service();
        let created_todo = create_sample_todo(&mut service, "To be deleted");

        let result = service.delete_todo(created_todo.id);

        match result {
            DeleteResult::Ok(deleted) => {
                assert!(deleted);
                match service.get_todo(created_todo.id) {
                    TodoResult::Err(e) => assert_eq!(e, "Todo not found"),
                    TodoResult::Ok(_) => panic!("Todo should be deleted"),
                }
            }
            DeleteResult::Err(e) => panic!("Expected success, got error: {}", e),
        }
    }

    #[test]
    fn test_delete_todo_not_found() {
        let mut service = create_test_service();

        let result = service.delete_todo(999);

        match result {
            DeleteResult::Ok(_) => panic!("Expected error for non-existent todo"),
            DeleteResult::Err(e) => assert_eq!(e, "Todo not found"),
        }
    }

    #[test]
    fn test_crud_workflow() {
        let mut service = create_test_service();

        let todo = create_sample_todo(&mut service, "Learn Rust");
        assert_eq!(todo.text, "Learn Rust");
        assert!(!todo.completed);

        match service.get_todo(todo.id) {
            TodoResult::Ok(retrieved) => assert_eq!(retrieved.id, todo.id),
            TodoResult::Err(e) => panic!("Should retrieve todo: {}", e),
        }

        let update_input = UpdateTodoInput {
            id: todo.id,
            text: Some("Master Rust".to_string()),
            completed: Some(true),
        };

        match service.update_todo(update_input) {
            TodoResult::Ok(updated) => {
                assert_eq!(updated.text, "Master Rust");
                assert!(updated.completed);
            }
            TodoResult::Err(e) => panic!("Should update todo: {}", e),
        }

        match service.delete_todo(todo.id) {
            DeleteResult::Ok(_) => {}
            DeleteResult::Err(e) => panic!("Should delete todo: {}", e),
        }

        match service.get_todo(todo.id) {
            TodoResult::Err(e) => assert_eq!(e, "Todo not found"),
            TodoResult::Ok(_) => panic!("Todo should be deleted"),
        }
    }

    #[test]
    fn test_get_todo_count() {
        let mut service = create_test_service();
        assert_eq!(service.get_todo_count(), 0);

        create_sample_todo(&mut service, "First");
        assert_eq!(service.get_todo_count(), 1);

        create_sample_todo(&mut service, "Second");
        assert_eq!(service.get_todo_count(), 2);

        service.delete_todo(1);
        assert_eq!(service.get_todo_count(), 1);
    }
}
