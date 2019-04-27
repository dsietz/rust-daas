use daas::sourcing;
use actix_web::{server};

pub fn main() {    
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    server::new( || {sourcing::service()})
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}