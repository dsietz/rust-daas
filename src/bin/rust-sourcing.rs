use daas::staging;
use actix_web::{server};

pub fn main() {
    server::new( || {staging::service()})
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}