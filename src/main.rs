extern crate failure;
extern crate notify;
extern crate sentry;

use failure::err_msg;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use sentry::integrations::failure::capture_error;
use sentry::integrations::panic::register_panic_handler;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

struct Config {
    root_folder: String,
}

fn main() {
    // uses SENTRY_DSN to create connection
    let _guard = sentry::init(());
    register_panic_handler();

    let config = get_default_config();

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.

    match watcher.watch(&config.root_folder, RecursiveMode::Recursive) {
        Ok(_) => (),
        Err(e) => {
            capture_error(&err_msg(e.to_string()));
            println!("watch error: {:?}", e);
        }
    }

    event_loop(rx);
}

fn event_loop(rx: Receiver<DebouncedEvent>) {
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => {
                capture_error(&err_msg(e.to_string()));
                println!("watch error: {:?}", e);
            }
        }
    }
}

fn get_default_config() -> Config {
    Config {
        root_folder: String::from("/bucket"),
    }
}
