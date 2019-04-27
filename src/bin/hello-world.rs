use daas::hello_world;
use actix_web::{server};

pub fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    server::new( || {hello_world::service()})
    .bind("127.0.0.1:7999")
    .expect("Can not bind to port 7999")
    .run();
}