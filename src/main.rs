#![allow(dead_code)]
#![allow(unused_variables)]

extern crate azure_sdk_for_rust;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate notify;
extern crate sentry;
#[macro_use]
extern crate log;
extern crate md5;
extern crate tokio_core;
extern crate url;
#[macro_use]
extern crate failure;

mod bucket;
mod event_handlers;
mod file_system;
mod storage;

use sentry::integrations::panic::register_panic_handler;
use std::borrow::Cow;
use std::env;

struct Config {
    root_folder: String,
}

fn main() {
    env_logger::init();
    sentry_config();
    register_panic_handler();

    bucket::start();
}

fn sentry_config() {
    trace!("sentry_config()");
    let sentry_dsn = env::var("SENTRY_DSN").expect("Set env variable SENTRY_DSN");
    trace!("sentry dsn - {}", sentry_dsn);
    let _guard = sentry::init((
        sentry_dsn,
        sentry::ClientOptions {
            release: Some(Cow::from("v0.1.0")),
            ..Default::default()
        },
    ));
}
