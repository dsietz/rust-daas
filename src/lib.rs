#[macro_use] extern crate serde_derive;
extern crate json;
extern crate base64;
extern crate actix_web;

use std::str;
use std::time::{SystemTime};
use base64::{decode};

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


//pub mod contract;
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
