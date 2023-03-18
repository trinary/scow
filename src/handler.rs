use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub(crate) struct DbDropGuard {
    db: Db,
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
    shared: Arc<Shared>,
}


impl Db {
    pub(crate) fn new() -> Db {
        Db {
            shared: Arc::new(Shared {
                state: Mutex::new(State {
                    entries: HashMap::new(),
                    servers: HashMap::new(),
                }),
            }),
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<String> {
        let state = self.shared.state.lock().unwrap();
        println!("i have some keys: {:?}", state.entries.keys());
        state.entries.get(key).map(|entry| entry.clone())
    }

    pub(crate) fn set(&self, key: String, value: String) {
        let mut state = self.shared.state.lock().unwrap();
        let _prev = state.entries.insert(key, value);
    }

    pub(crate) fn add_server(&self, address: String, name: String) {
        let mut state = self.shared.state.lock().unwrap();
        state.servers.insert(address, name);
    }
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, String>,
    servers: HashMap<String, String>,
}
