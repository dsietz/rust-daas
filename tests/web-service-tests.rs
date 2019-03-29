extern crate actix_web;
extern crate bytes;
extern crate json;

use daas::staging;
use actix_web::*;
use http::header;
use bytes::Bytes;
use rand::Rng;
use serde_json::{Value};

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

#[test]
fn test_stage_data_ok(){
    let uri = daas::staging::get_service_path()
        .replace("{category}","order")
        .replace("{subcategory}","clothing")
        .replace("{source_name}","iStore")
        .replace("{source_uid}","5000");  
    let mut srv =actix_web::test::TestServer::new(|app| {
                        app.resource(
                            &daas::staging::get_service_path(),
                            |r| r.post().with(daas::staging::stage)
                        );
                    });
    let request = srv.post()
                    .uri(srv.url(&uri))
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("Authorization","Basic Zm9vOmJhcg==")
                    .body(r#"{"data":"Hello, world!"}"#)
                    .unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    // read response
    let bytes = srv.execute(response.body()).unwrap();
    let body: Value = serde_json::from_str(&String::from_utf8(bytes.to_vec()).unwrap()).unwrap();

    assert_eq!(body["status"], "OK".to_string());
}


#[test]
fn test_stage_data_bad_parameter(){
let uri = daas::staging::get_service_path()
        .replace("{category}","order")
        .replace("{subcategory}","clothing")
        .replace("{source_name}","iStore")
        .replace("{source_uid}","word");  
    let mut srv =actix_web::test::TestServer::new(|app| {
                        app.resource(
                            &daas::staging::get_service_path(),
                            |r| r.post().with(daas::staging::stage)
                        );
                    });
    let request = srv.post()
                    .uri(srv.url(&uri))
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("Authorization","Basic Zm9vOmJhcg==")
                    .body(r#"{"data":"Hello, world!"}"#)
                    .unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
}

#[test]
fn test_stage_data_bad_payload(){
    let mut rng = rand::thread_rng();
    let uri = daas::staging::get_service_path()
        .replace("{category}","order")
        .replace("{subcategory}","clothing")
        .replace("{source_name}","iStore")
        .replace("{source_uid}", "112233");  
    let mut srv =actix_web::test::TestServer::new(|app| {
                        app.resource(
                            &daas::staging::get_service_path(),
                            |r| r.post().with(daas::staging::stage)
                        );
                    });
    let request = srv.post()
                    .uri(srv.url(&uri))
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("Authorization","Basic Zm9vOmJhcg==")
                    .body(r#"{"data":...}"#)
                    .unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
}