extern crate failure;
extern crate notify;
extern crate sentry;

use failure::err_msg;
use notify::{watcher, RecursiveMode, Watcher};
use sentry::integrations::failure::capture_error;
use sentry::integrations::panic::register_panic_handler;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    // uses SENTRY_DSN to create connection
    let _guard = sentry::init(());
    register_panic_handler();

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.

    match watcher.watch("c:\\bucket", RecursiveMode::Recursive) {
        Ok(_) => (),
        Err(e) => {
            capture_error(&err_msg(e.to_string()));
            println!("watch error: {:?}", e);
        }
    }

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
