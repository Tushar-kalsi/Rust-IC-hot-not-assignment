use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

use crate::types::{Todo, TodoId};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type TodoStorage = StableBTreeMap<TodoId, Todo, Memory>;
pub type IdStorage = StableBTreeMap<u8, TodoId, Memory>;

const TODO_COUNTER_KEY: u8 = 0;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static TODO_STORE: RefCell<TodoStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static ID_STORE: RefCell<IdStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

pub fn with_todo_store<R>(f: impl FnOnce(&TodoStorage) -> R) -> R {
    TODO_STORE.with(|store| f(&store.borrow()))
}

pub fn with_todo_store_mut<R>(f: impl FnOnce(&mut TodoStorage) -> R) -> R {
    TODO_STORE.with(|store| f(&mut store.borrow_mut()))
}

pub fn get_next_todo_id() -> TodoId {
    ID_STORE.with(|store| {
        let mut store = store.borrow_mut();
        let current_id = store.get(&TODO_COUNTER_KEY).unwrap_or(0);
        let next_id = current_id + 1;
        store.insert(TODO_COUNTER_KEY, next_id);
        next_id
    })
}

pub fn init_storage() {
    ID_STORE.with(|store| {
        let mut store = store.borrow_mut();
        if store.get(&TODO_COUNTER_KEY).is_none() {
            store.insert(TODO_COUNTER_KEY, 0);
        }
    });
}
