extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate json;
extern crate base64;
extern crate actix_web;
extern crate roadrunner;
extern crate tokio_core;
extern crate hyper;
extern crate rand;
extern crate futures;
extern crate actix_web_httpauth;
extern crate kafka;

use std::str;
use std::time::{SystemTime};
use actix_web::middleware::Logger;

/// globals
pub static DELIMITER: &str = "-";
pub static KAFKA_BROKERS: &str = "localhost:9092";
static ZOOKEEPER_DIR: &str = "C:/kafka/kafka_2.12-2.2.0";
static VER: &str = "v1";

/// methods
fn get_unix_now() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) =>n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}


pub mod hello_world;
pub mod daas;
pub mod couchdb;
pub mod broker;
pub mod sourcing;
pub mod processor;
pub mod data_service;