use super::event_handlers::{CreatedEvent, EventHandler, RemovedEvent, UpdatedEvent};
use super::file_system;
use super::storage;
use failure::err_msg;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use sentry::integrations::failure::capture_error;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct Config {
    pub root_folder: String,
    pub storage_account: String,
    pub account_key: String,
    pub root_container_name: String,
}

pub fn start() {
    let config = get_default_config();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    match watcher.watch(&config.root_folder, RecursiveMode::Recursive) {
        Ok(_) => (),
        Err(e) => {
            capture_error(&err_msg(e.to_string()));
            trace!("watch error: {:?}", e);
        }
    }

    event_loop(&rx, &config);
}

fn event_loop(rx: &Receiver<DebouncedEvent>, config: &Config) {
    let storage = storage::AzureStorage::new(config);
    let file_system = file_system::LocalFileSystem::new(config);
    let evts = initialise_event_handlers(&storage, &file_system);

    for event in rx {
        route_event(&event, &evts);
    }
}

fn route_event(evt: &DebouncedEvent, evts: &EventHandler) {
    match evt {
        DebouncedEvent::Create(p) => evts.call("create", p),
        DebouncedEvent::Remove(p) => evts.call("remove", p),
        DebouncedEvent::Write(p) => evts.call("update", p),
        _ => (), // only interested in the Create, Remove and Write events
    }
}

fn get_default_config() -> Config {
    Config {
        root_folder: std::env::var("ROOT_FOLDER").expect("Set env variable ROOT_FOLDER"),
        storage_account: std::env::var("STORAGE_ACCOUNT")
            .expect("Set env variable STORAGE_ACCOUNT"),
        account_key: std::env::var("STORAGE_MASTER_KEY")
            .expect("Set env variable STORAGE_MASTER_KEY"),
        root_container_name: std::env::var("STORAGE_CONTAINER")
            .expect("Set env variable STORAGE_CONTAINER"),
    }
}

fn initialise_event_handlers<'a>(
    storage: &'a storage::Storage,
    file_system: &'a file_system::FileSystem,
) -> EventHandler<'a> {
    let mut e = EventHandler::new(storage, file_system);
    e.add("create", &CreatedEvent {});
    e.add("remove", &RemovedEvent {});
    e.add("update", &UpdatedEvent {});
    e
}

#[cfg(test)]
mod tests {
    use super::*;
    use event_handlers::PathEventHandler;
    use std::cell::RefCell;
    use std::path::PathBuf;

    struct MockStorage {}

    impl MockStorage {
        fn new() -> MockStorage {
            MockStorage {}
        }
    }

    impl storage::Storage for MockStorage {
        fn upload(&self, blob_name: &str, data: Vec<u8>) {}
        fn download(&self, p: &PathBuf) {}
        fn delete(&self, blob_name: &str) {}
    }

    struct MockFileSystem {
        get_blob_name_called: RefCell<bool>,
        get_file_contents_called: RefCell<bool>,
    }

    impl MockFileSystem {
        fn new() -> MockFileSystem {
            MockFileSystem {
                get_blob_name_called: RefCell::new(false),
                get_file_contents_called: RefCell::new(false),
            }
        }
    }

    impl file_system::FileSystem for MockFileSystem {
        fn get_blob_name(&self, p: &PathBuf) -> String {
            *self.get_blob_name_called.borrow_mut() = true;
            String::from("")
        }
        fn get_file_contents(&self, p: &PathBuf) -> Vec<u8> {
            *self.get_file_contents_called.borrow_mut() = true;
            Vec::new()
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
        fn handle(
            &self,
            _path: &PathBuf,
            _storage: &storage::Storage,
            _file_system: &file_system::FileSystem,
        ) {
            *self.called.borrow_mut() = true;
        }
    }

    #[test]
    fn test_create_event_calls_create_handler() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        let mock_create_handler = MockPathEventHandler::new();
        let mock_remove_handler = MockPathEventHandler::new();
        let mock_update_handler = MockPathEventHandler::new();
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);
        e.add("create", &mock_create_handler);
        e.add("remove", &mock_remove_handler);
        e.add("update", &mock_update_handler);

        route_event(&DebouncedEvent::Create(PathBuf::new()), &e);

        assert_eq!(*mock_create_handler.called.borrow(), true);
        assert_eq!(*mock_remove_handler.called.borrow(), false);
        assert_eq!(*mock_update_handler.called.borrow(), false);
    }

    #[test]
    fn test_remove_event_calls_remove_handler() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        let mock_create_handler = MockPathEventHandler::new();
        let mock_remove_handler = MockPathEventHandler::new();
        let mock_update_handler = MockPathEventHandler::new();
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);
        e.add("create", &mock_create_handler);
        e.add("remove", &mock_remove_handler);
        e.add("update", &mock_update_handler);

        route_event(&DebouncedEvent::Remove(PathBuf::new()), &e);

        assert_eq!(*mock_create_handler.called.borrow(), false);
        assert_eq!(*mock_remove_handler.called.borrow(), true);
        assert_eq!(*mock_update_handler.called.borrow(), false);
    }

    #[test]
    fn test_write_event_calls_update_handler() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        let mock_create_handler = MockPathEventHandler::new();
        let mock_remove_handler = MockPathEventHandler::new();
        let mock_update_handler = MockPathEventHandler::new();
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);
        e.add("create", &mock_create_handler);
        e.add("remove", &mock_remove_handler);
        e.add("update", &mock_update_handler);

        route_event(&DebouncedEvent::Write(PathBuf::new()), &e);

        assert_eq!(*mock_create_handler.called.borrow(), false);
        assert_eq!(*mock_remove_handler.called.borrow(), false);
        assert_eq!(*mock_update_handler.called.borrow(), true);
    }

    #[test]
    fn test_ignored_event_does_not_call_event_handler() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        let mock_create_handler = MockPathEventHandler::new();
        let mock_remove_handler = MockPathEventHandler::new();
        let mock_update_handler = MockPathEventHandler::new();
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);
        e.add("create", &mock_create_handler);
        e.add("remove", &mock_remove_handler);
        e.add("update", &mock_update_handler);

        route_event(&DebouncedEvent::NoticeWrite(PathBuf::new()), &e);

        assert_eq!(*mock_create_handler.called.borrow(), false);
        assert_eq!(*mock_remove_handler.called.borrow(), false);
        assert_eq!(*mock_update_handler.called.borrow(), false);
    }
}
