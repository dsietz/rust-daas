#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate rocket_contrib;

use daas::staging;
use rocket::local::Client;
use rocket::http::{ContentType, Header, Status};

#[test]
fn test_status_code_ok(){
    let client = Client::new(staging::service()).expect("valid rocekt instance");
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello World!".into()));
}

#[test]
fn test_stage_data_ok(){
    let client = Client::new(staging::service()).expect("valid rocket instance");
    let mut response = client.post("/stage/order/clothes/iStore/5000")
        .header(ContentType::JSON)
        .header(Header::new("Authorization","Basic Zm9vOmJhcg=="))
        .body(r#"{ "data": "Hello, world!" }"#)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("{\"status\":\"OK\"}".into()));
}

#[test]
fn test_stage_data_bad_json(){
    let client = Client::new(staging::service()).expect("valid rocket instance");
    let response = client.post("/stage/order/clothes/iStore/5000")
        .header(ContentType::JSON)
        .header(Header::new("Authorization","Basic Zm9vOmJhcg=="))
        .body(r#"{ "data": ... }"#)
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_stage_data_missing_data(){
    let client = Client::new(staging::service()).expect("valid rocket instance");
    let response = client.post("/stage/order/clothes/iStore/5000")
        .header(ContentType::JSON)
        .header(Header::new("Authorization","Basic Zm9vOmJhcg=="))
        .body(r#"{ "created_timestamp": 1552681300 }"#)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_stage_data_auth_ok(){
    let client = Client::new(staging::service()).expect("valid rocket instance");
    let mut response = client.post("/stage/order/clothes/iStore/5000")
        .header(ContentType::JSON)
        .header(Header::new("Authorization","Basic Zm9vOmJhcg=="))
        .body(r#"{ "data": "Hello, world!" }"#)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("{\"status\":\"OK\"}".into()));
}