use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{Notify};
use crate::connection::Connection;


#[derive(Debug, Clone)]
pub(crate) struct DbDropGuard {
    db: Db
}

impl DbDropGuard {
    pub(crate) fn new() -> DbDropGuard {
        DbDropGuard { db: Db::new() }
    }

    pub(crate) fn db(&self) -> Db {
        self.db.clone()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Db {
    shared: Arc<Shared>
}

impl Db {
    pub(crate) fn new() -> Db {
        Db {
            shared: Arc::new(Shared {
                state: Mutex::new(State {
                    entries: HashMap::new(),
                    shutdown: false
                }),
                background_task: Notify::new(),
            })
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<String> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).map(|entry| entry.clone())
    }

    pub(crate) fn set(&self, key: String, value: String) {
        let mut state = self.shared.state.lock().unwrap();
        let prev = state.entries.insert(
            key, value
        );
    }
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
    background_task: Notify
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, String>,
    shutdown: bool,
}