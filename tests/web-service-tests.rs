#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
#[macro_use] 
extern crate rocket_contrib;

use daas::staging;
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn test_status_code_ok(){
    let client = Client::new(staging::service()).expect("valid rocekt instance");
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello World!".into()));
}

#[test]
fn test_stage_data_ok(){
    let client = Client::new(staging::service()).expect("valid rocekt instance");
    let mut response = client.post("/stage/product/clothes/iStore/5000")
        .header(ContentType::JSON)
        .body(r#"{ "id": 100, "data": "Hello, world!" }"#)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("{\"status\":\"ok\"}".into()));
}

#[test]
fn test_stage_data_bad_json(){
    let client = Client::new(staging::service()).expect("valid rocekt instance");
    let response = client.post("/stage/product/clothes/iStore/5000")
        .header(ContentType::JSON)
        .body(r#"{ "id": 100, "data": ... }"#)
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}