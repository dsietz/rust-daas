extern crate actix_web;
extern crate bytes;
extern crate json;

use daas::staging;
use actix_web::*;
use http::header;
use bytes::Bytes;

#[test]
fn test_status_code_ok() {
    let mut srv =actix_web::test::TestServer::new(|app| app.handler(staging::index));
    let request = srv.get().uri(srv.url("/").as_str()).finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    // read response
    let bytes = srv.execute(response.body()).unwrap();
    assert_eq!(bytes, Bytes::from_static("Hello World!".as_ref()));
}

#[ignore]
#[test]
fn test_stage_data_ok(){
    let mut srv =actix_web::test::TestServer::new(|app| app.handler(staging::stage));
    let request = srv.get()
                    .uri(srv.url("/stage/v1/order/clothing/iStore/5000").as_str())
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("Authorization","Basic Zm9vOmJhcg==")
                    .json("{\"data\":\"Hello, world!\"}")
                    //.finish()
                    .unwrap();
    let response = srv.execute(request.send()).unwrap();
    println!("STATUS: {}",response.status());
    assert!(response.status().is_success());

    // read response
    let bytes = srv.execute(response.body()).unwrap();
    assert_eq!(bytes, Bytes::from_static("{\"status\":\"OK\"}".as_ref()));
}

/*
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
*/