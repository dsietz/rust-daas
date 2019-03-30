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
use base64::{decode};


/// globals
static ZOOKEEPER_DIR: &str = "C:/kafka/kafka_2.12-2.2.0";
static DELIMITER: &str = "-";
static VER: &str = "v1";


/// methods
fn get_unix_now() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) =>n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub mod daas;
pub mod couchdb;
pub mod broker;
pub mod staging;

//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(1,1);
    }
}
