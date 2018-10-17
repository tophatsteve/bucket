extern crate failure;
extern crate notify;
extern crate sentry;

use super::event_handlers::{CreatedEvent, EventHandler, RemovedEvent, UpdatedEvent};
use super::storage;
use failure::err_msg;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use sentry::integrations::failure::capture_error;
// use sentry::integrations::panic::register_panic_handler;
// use std::borrow::Cow;
// use std::env;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

struct Config {
    root_folder: String,
}

pub fn start() {
    let config = get_default_config();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    match watcher.watch(&config.root_folder, RecursiveMode::Recursive) {
        Ok(_) => (),
        Err(e) => {
            capture_error(&err_msg(e.to_string()));
            println!("watch error: {:?}", e);
        }
    }

    event_loop(&rx);
}

fn event_loop(rx: &Receiver<DebouncedEvent>) {
    let storage = storage::AzureStorage {};
    let evts = initialise_event_handlers(&storage);

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
        root_folder: String::from("/bucket"),
    }
}

// storage needs to live as long as returned EventHandler
fn initialise_event_handlers<'a>(storage: &'a storage::Storage) -> EventHandler<'a> {
    let mut e = EventHandler::new(storage);
    e.add("create", &CreatedEvent {});
    e.add("remove", &RemovedEvent {});
    e.add("update", &UpdatedEvent {});
    e
}
