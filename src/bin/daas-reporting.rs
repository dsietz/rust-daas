use daas::data_service;
use actix_web::{server};

pub fn main() {    
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    server::new( || {data_service::service()})
    .bind("127.0.0.1:8001")
    .expect("Can not bind to port 8001")
    .run();
}