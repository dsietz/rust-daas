#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate json;
extern crate base64;

use std::str;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use base64::{decode};

/// globals
static SERVICE_ROOT: &str = "/v1";

/// structures
#[derive(Debug)]
pub struct HdrBase64(String);

/// methods
pub fn get_author(auth: &str) -> Option<String>{
    match is_valid_auth(auth) {
        true => {
            let encoded = auth.replacen("Basic ", "", 1);

            match decode(&encoded) {
                Ok(decoded) => {
                    let decoded = str::from_utf8(&decoded).unwrap().to_string();
                    let parts: Vec<&str> = decoded.split(':').collect();

                    Some(parts[1].to_string())
                },
                Err(_err) => {
                    //panic!("Warning: Bad encoded Authorization Header value: {}", encoded)
                    None
                }
            }
        },
        false => None
    }
}

fn get_unix_now() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) =>n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn is_valid_auth(auth: &str) -> bool {
    auth.contains("Basic ")
}

/// implementations
impl<'a, 'r> FromRequest<'a, 'r> for HdrBase64 {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<HdrBase64, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        if !is_valid_auth(keys[0]) {
            return Outcome::Forward(());
        }

        return Outcome::Success(HdrBase64(key.to_string()));
    }
}

impl fmt::Display for HdrBase64 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod contract;
pub mod staging;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_auth_ok() {
        assert_eq!(is_valid_auth("Basic Zm9vOmJhcg=="), true);
    }

    #[test]
    fn test_valid_auth_bad() {
        assert_eq!(is_valid_auth("Zm9vOmJhcg=="), false);
    }

    #[test]
    fn test_get_author_ok() {
        let author = get_author("Basic Zm9vOmJhcg==");
        assert!(author.is_some());
        assert_eq!(author.unwrap(), "bar");
    }

    #[test]
    fn test_get_author_bad() {
        let author = get_author("Basic foo:bar");
        assert!(author.is_none());
    }
}
