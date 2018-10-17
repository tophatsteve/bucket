#![allow(dead_code)]
#![allow(unused_variables)]

extern crate failure;
extern crate notify;
extern crate sentry;

mod bucket;
mod event_handlers;
mod storage;

use sentry::integrations::panic::register_panic_handler;
use std::borrow::Cow;
use std::env;

struct Config {
    root_folder: String,
}

fn main() {
    sentry_config();
    register_panic_handler();

    bucket::start();
}

fn sentry_config() {
    let sentry_dsn = env::var("SENTRY_DSN").unwrap();
    let _guard = sentry::init((
        sentry_dsn,
        sentry::ClientOptions {
            release: Some(Cow::from("v0.1.0")),
            ..Default::default()
        },
    ));
}
