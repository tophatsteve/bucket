#![allow(dead_code)]

use super::storage;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct EventHandler<'a> {
    event_handlers: HashMap<&'a str, &'a PathEventHandler>,
    storage: &'a storage::Storage,
}

impl<'a> EventHandler<'a> {
    pub fn new(storage: &'a storage::Storage) -> EventHandler<'a> {
        EventHandler {
            event_handlers: HashMap::new(),
            storage: storage,
        }
    }

    pub fn add(&mut self, event_name: &'a str, event_handler: &'a PathEventHandler) {
        self.event_handlers.insert(event_name, event_handler);
    }

    pub fn call(&self, event_name: &str, path: &PathBuf) {
        if let Some(f) = self.event_handlers.get(event_name) {
            println!("Calling event for {}", event_name);
            f.handle(path, self.storage);
        }
    }
}

pub trait PathEventHandler {
    fn handle(&self, path: &PathBuf, storage: &storage::Storage);
}

pub struct CreatedEvent {}

impl PathEventHandler for CreatedEvent {
    fn handle(&self, path: &PathBuf, storage: &storage::Storage) {
        storage.upload(path);
    }
}

pub struct RemovedEvent {}

impl PathEventHandler for RemovedEvent {
    fn handle(&self, path: &PathBuf, _storage: &storage::Storage) {
        println!("Called RemovedEvent with {:?}", path);
    }
}

pub struct UpdatedEvent {}

impl PathEventHandler for UpdatedEvent {
    fn handle(&self, path: &PathBuf, _storage: &storage::Storage) {
        println!("Called UpdatedEvent with {:?}", path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockStorage {
        upload_called: RefCell<bool>,
        download_called: RefCell<bool>,
    }

    impl MockStorage {
        fn new() -> MockStorage {
            MockStorage {
                upload_called: RefCell::new(false),
                download_called: RefCell::new(false),
            }
        }
    }

    impl storage::Storage for MockStorage {
        fn upload(&self, p: &PathBuf) {
            *self.upload_called.borrow_mut() = true;
        }
        fn download(&self, p: &PathBuf) {
            *self.download_called.borrow_mut() = true;
        }
    }

    struct MockPathEventHandler {
        called: RefCell<bool>,
    }

    impl MockPathEventHandler {
        fn new() -> MockPathEventHandler {
            MockPathEventHandler {
                called: RefCell::new(false),
            }
        }
    }

    impl PathEventHandler for MockPathEventHandler {
        fn handle(&self, _path: &PathBuf, _storage: &storage::Storage) {
            *self.called.borrow_mut() = true;
        }
    }

    #[test]
    fn test_handler_added_to_list() {
        let mock_storage = MockStorage::new();
        let mock_event_handler = MockPathEventHandler::new();
        let mut e = EventHandler::new(&mock_storage);

        e.add("mock", &mock_event_handler);

        assert!(e.event_handlers.get("mock").is_some());
    }

    #[test]
    fn test_handler_is_called() {
        let mock_storage = MockStorage::new();
        let mock_event_handler = MockPathEventHandler::new();
        let mut e = EventHandler::new(&mock_storage);

        e.add("mock", &mock_event_handler);
        e.call("mock", &PathBuf::new());

        assert!(*mock_event_handler.called.borrow());
    }

    #[test]
    fn test_create_event_calls_storage_upload() {
        let mock_storage = MockStorage::new();
        let mut e = EventHandler::new(&mock_storage);

        e.add("create", &CreatedEvent {});
        e.call("create", &PathBuf::new());

        assert!(*mock_storage.upload_called.borrow());
    }
}
