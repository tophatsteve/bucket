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
            f.handle(path);
        }
    }
}

pub trait PathEventHandler {
    fn handle(&self, path: &PathBuf);
}

pub struct CreatedEvent {}

impl PathEventHandler for CreatedEvent {
    fn handle(&self, path: &PathBuf) {
        println!("Called CreatedEvent with {:?}", path);
    }
}

pub struct RemovedEvent {}

impl PathEventHandler for RemovedEvent {
    fn handle(&self, path: &PathBuf) {
        println!("Called RemovedEvent with {:?}", path);
    }
}

pub struct UpdatedEvent {}

impl PathEventHandler for UpdatedEvent {
    fn handle(&self, path: &PathBuf) {
        println!("Called UpdatedEvent with {:?}", path);
    }
}
