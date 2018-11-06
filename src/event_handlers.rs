use super::file_system;
use super::storage;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct EventHandler<'a> {
    event_handlers: HashMap<&'a str, &'a PathEventHandler>,
    storage: &'a storage::Storage,
    file_system: &'a file_system::FileSystem,
}

impl<'a> EventHandler<'a> {
    pub fn new(
        storage: &'a storage::Storage,
        file_system: &'a file_system::FileSystem,
    ) -> EventHandler<'a> {
        EventHandler {
            event_handlers: HashMap::new(),
            storage,
            file_system,
        }
    }

    pub fn add(&mut self, event_name: &'a str, event_handler: &'a PathEventHandler) {
        self.event_handlers.insert(event_name, event_handler);
    }

    pub fn call(&self, event_name: &str, path: &PathBuf) {
        if let Some(f) = self.event_handlers.get(event_name) {
            trace!("Calling event for {}", event_name);
            f.handle(path, self.storage, self.file_system);
        }
    }
}

pub trait PathEventHandler {
    fn handle(
        &self,
        path: &PathBuf,
        storage: &storage::Storage,
        file_system: &file_system::FileSystem,
    );
}

pub struct CreatedEvent {}

impl PathEventHandler for CreatedEvent {
    fn handle(
        &self,
        path: &PathBuf,
        storage: &storage::Storage,
        file_system: &file_system::FileSystem,
    ) {
        if path.is_dir() {
            return;
        }
        let blob_name = file_system.get_blob_name(path);
        let file_content = file_system.get_file_contents(path);
        storage.upload(&blob_name, file_content);
    }
}

pub struct RemovedEvent {}

impl PathEventHandler for RemovedEvent {
    fn handle(
        &self,
        path: &PathBuf,
        storage: &storage::Storage,
        file_system: &file_system::FileSystem,
    ) {
        let blob_name = file_system.get_blob_name(path);

        match storage.delete(&blob_name) {
            Err(storage::StorageError::PathNotFound) => {
                let blobs_to_delete = storage.list_folder_blobs(&blob_name);
                for blob in blobs_to_delete {
                    match storage.delete(&file_system.encode_file_name(&blob_name)) {
                        Err(e) => trace!("{}", e),
                        Ok(_) => (),
                    }
                }
            }
            Err(e) => trace!("{}", e),
            Ok(_) => (),
        };
    }
}

pub struct UpdatedEvent {}

impl PathEventHandler for UpdatedEvent {
    fn handle(
        &self,
        path: &PathBuf,
        _storage: &storage::Storage,
        _file_system: &file_system::FileSystem,
    ) {
        trace!("Called UpdatedEvent with {:?}", path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockStorage {
        upload_called: RefCell<bool>,
        download_called: RefCell<bool>,
        delete_called: RefCell<bool>,
        list_folder_blobs_called: RefCell<bool>,
        return_path_not_found_error: RefCell<bool>,
    }

    impl MockStorage {
        fn new() -> MockStorage {
            MockStorage {
                upload_called: RefCell::new(false),
                download_called: RefCell::new(false),
                delete_called: RefCell::new(false),
                list_folder_blobs_called: RefCell::new(false),
                return_path_not_found_error: RefCell::new(false),
            }
        }

        fn set_return_path_not_found_error(&self, f: bool) {
            *self.return_path_not_found_error.borrow_mut() = f;
        }
    }

    impl storage::Storage for MockStorage {
        fn upload(&self, blob_name: &str, data: Vec<u8>) {
            *self.upload_called.borrow_mut() = true;
        }
        fn download(&self, p: &PathBuf) {
            *self.download_called.borrow_mut() = true;
        }
        fn delete(&self, blob_name: &str) -> Result<(), storage::StorageError> {
            *self.delete_called.borrow_mut() = true;

            if *self.return_path_not_found_error.borrow() {
                return Err(storage::StorageError::PathNotFound);
            }

            Ok(())
        }
        fn list_folder_blobs(&self, blob_name: &str) -> Vec<String> {
            *self.list_folder_blobs_called.borrow_mut() = true;
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

    struct MockFileSystem {
        get_blob_name_called: RefCell<bool>,
        get_file_contents_called: RefCell<bool>,
        encode_file_name_called: RefCell<bool>,
    }

    impl MockFileSystem {
        fn new() -> MockFileSystem {
            MockFileSystem {
                get_blob_name_called: RefCell::new(false),
                get_file_contents_called: RefCell::new(false),
                encode_file_name_called: RefCell::new(false),
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
        fn encode_file_name(&self, f: &str) -> String {
            *self.encode_file_name_called.borrow_mut() = true;
            String::from("")
        }
    }

    #[test]
    fn test_handler_added_to_list() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        let mock_event_handler = MockPathEventHandler::new();
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);

        e.add("mock", &mock_event_handler);

        assert!(e.event_handlers.get("mock").is_some());
    }

    #[test]
    fn test_handler_is_called() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        let mock_event_handler = MockPathEventHandler::new();
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);

        e.add("mock", &mock_event_handler);
        e.call("mock", &PathBuf::new());

        assert!(*mock_event_handler.called.borrow());
    }

    #[test]
    fn test_create_event_calls_storage_upload() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);

        e.add("create", &CreatedEvent {});
        e.call("create", &PathBuf::new());

        assert!(*mock_storage.upload_called.borrow());
    }

    #[test]
    fn test_create_handler_is_not_called_for_directories() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);

        e.add("create", &CreatedEvent {});
        e.call("create", &PathBuf::from("/"));

        assert_eq!(*mock_storage.upload_called.borrow(), false);
    }

    #[test]
    fn test_remove_event_calls_storage_delete() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);

        e.add("remove", &RemovedEvent {});
        e.call("remove", &PathBuf::new());

        assert!(*mock_storage.delete_called.borrow());
    }

    #[test]
    fn test_remove_non_existing_file_calls_list_folder_blobs() {
        let mock_file_system = MockFileSystem::new();
        let mock_storage = MockStorage::new();
        mock_storage.set_return_path_not_found_error(true);
        let mut e = EventHandler::new(&mock_storage, &mock_file_system);

        e.add("remove", &RemovedEvent {});
        e.call("remove", &PathBuf::new());

        assert!(*mock_storage.list_folder_blobs_called.borrow());
    }
}
