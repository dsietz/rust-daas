use daas::data_service;
use actix_web::{server};

pub fn main() {
    server::new( || {data_service::service()})
    .bind("127.0.0.1:8001")
    .expect("Can not bind to port 8001")
    .run();
}