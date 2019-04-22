use super::*;
use actix_web::{App, http, HttpRequest, HttpResponse, Path, Responder};
use actix_web::http::header::Header;
use actix_web_httpauth::extractors::basic::BasicAuth;
use super::daas::{DaaSDoc};
use super::couchdb::{CouchDB};
use broker::*;
use std::thread;
use std::time::Duration;
use super::daas::*;

/// globals
static DB_NAME: &str = "consuming";
static DB_USER: &str = "admin";
static DB_PSWRD: &str = "password";
static CATEGORY: &str = "history";
static SUBCATEGORY: &str = "status";

#[derive(Deserialize)]
pub struct Info {
    source_name: String,
    source_uid: usize,
}

pub fn get_service_root() -> String {
    format!("/data/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/history/status/{source_name}/{source_uid}"
}

pub fn status_history(auth: BasicAuth, params: Path<Info>, _req: HttpRequest) -> HttpResponse {
    let id = daas::make_id(CATEGORY.to_string(), SUBCATEGORY.to_string(), params.source_name.clone(), params.source_uid as usize);        
    println!("ID: {}", id);

    let couch = CouchDB::new(DB_USER.to_string(), DB_PSWRD.to_string());
// http://localhost:5984/consuming/_design/history/_view/status-duration?reduce=true&group=true&skip=0&limit=21
    let reply = thread::spawn(move || {
            match couch.get_doc_by_id(DB_NAME.to_string(), id) {
                Ok(mut doc) => {
                    //let results = doc.data_obj_as_ref().as_str().unwrap().to_owned();
                    println!("Doc: {:?}", doc.data_obj_as_ref());
                    r#"{"status":"Ok"}"#.to_string()
                },
                _ => {
                    r#"{"error":"Could not find the document!"}"#.to_string()
                },
            }
        });

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(reply.join().unwrap())   
}

pub fn service() -> App {
    let app = App::new()
                .resource(
                    &get_service_path(),
                    |r| r.get().with(status_history));
    app
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/data/{}", VER));
    }
}
