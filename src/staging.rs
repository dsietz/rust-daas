use super::*;
use actix_web::{App, http, HttpRequest, HttpResponse, Path, Responder};
use actix_web::http::header::Header;
use actix_web_httpauth::extractors::basic::BasicAuth;
use super::daas::DaaSDoc;
use super::couchdb::{CouchDB};
use broker::*;
use std::thread;
use std::time::Duration;

/// globals
static DB_NAME: &str = "test";
static DB_USER: &str = "admin";
static DB_PSWRD: &str = "password";

#[derive(Deserialize)]
pub struct Info {
    category: String,
    subcategory: String,
    source_name: String,
    source_uid: usize,
}

#[derive(Deserialize, Debug)]
pub struct Bdy {
    quantitiy: usize,
    status: String,
}

pub fn get_service_root() -> String {
    format!("/stage/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/{category}/{subcategory}/{source_name}/{source_uid}"
}

fn process_data(couch: CouchDB, id: String, topic: String) -> Result<bool, String>{
    match couch.get_doc_by_id(DB_NAME.to_string(), id) {
        Ok(mut doc) => {
            match broker::produce_message(&doc.serialize().as_bytes(), &topic.clone(), vec!("localhost:9092".to_string())) {
                Ok(_v) => {
                    doc.process_ind = true;
                    couch.upsert_doc(DB_NAME.to_string(), doc).unwrap();

                    Ok(true)
                },
                _ => Err("Could not broker document".to_string())
            }
        },
        _ => Err("Could not find document.".to_string())
    }
}

pub fn index(_req: &HttpRequest) -> impl Responder {
    "Hello World!".to_string()
}

//https://docs.rs/actix-web-httpauth/0.1.0/actix_web_httpauth/headers/authorization/struct.Authorization.html
pub fn stage(auth: BasicAuth, params: Path<Info>, body: String, _req: HttpRequest) -> HttpResponse {
    let cat: String = params.category.clone();
    let subcat: String = params.subcategory.clone();
    let srcnme: String = params.source_name.clone();
    let srcuid: usize = params.source_uid;

    let data = match serde_json::from_str(&body) {
        Ok(d) => d,
        _ => {
            return HttpResponse::BadRequest()
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(r#"{"error":"Bad Json"}"#) 
        },
    };

    let topic = format!("{}{}{}{}{}", cat.clone(), DELIMITER, subcat.clone(), DELIMITER, srcnme.clone());
    //let msg = broker::produce_message("hello message".as_bytes(), &topic.clone(), vec!("localhost:9092".to_string()));
    
    let doc = DaaSDoc::new(srcnme, srcuid, cat, subcat, auth.username().to_string(), data);
    let doc_id = doc._id.clone();
    let couch = CouchDB::new(DB_USER.to_string(), DB_PSWRD.to_string());
    let save = thread::spawn(move || {
            match couch.upsert_doc(DB_NAME.to_string(), doc) {
                Ok(_rslt) => {
                    //process_data(CouchDB::new(DB_USER.to_string(), DB_PSWRD.to_string()), doc_id, topic).unwrap();
                    let _process = thread::spawn(move || {
                        process_data(CouchDB::new(DB_USER.to_string(), DB_PSWRD.to_string()), doc_id, topic).unwrap();
                    });
                    r#"{"status":"OK"}"#
                },
                _ => {
                    r#"{"error":"Could not save document!"}"#
                },
            }
        });

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(save.join().unwrap())    
}

pub fn service() -> App {
    let app = App::new()
                .resource(
                    "/", 
                    |r| r.get().f(index))
                .resource(
                    &(get_service_root() + "/{category}/{subcategory}/{source_name}/{source_uid}"),
                    |r| r.post().with(stage));
    app
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/stage/{}", VER));
    }
}
