use super::*;
use actix_web::{App, http, HttpRequest, HttpResponse, Path, Responder, Result};
use actix_web::http::header::Header;
use actix_web_httpauth::extractors::basic::BasicAuth;
use super::daas::DaaSDoc;
use super::couchdb::{CouchDB};
use std::thread;

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

pub fn index(_req: &HttpRequest) -> impl Responder {
    "Hello World!".to_string()
}

//https://docs.rs/actix-web-httpauth/0.1.0/actix_web_httpauth/headers/authorization/struct.Authorization.html
pub fn stage(auth: BasicAuth, params: Path<Info>, body: String, req: HttpRequest) -> HttpResponse {
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

    let doc = DaaSDoc::new(srcnme, srcuid, cat, subcat, auth.username().to_string(), data);
    let couch = CouchDB::new("admin".to_string(), "password".to_string());
    let save = thread::spawn(move || {
            match couch.upsert_doc("test".to_string(),doc) {
                Ok(_rslt) => {
                    r#"{"status":"OK"}"#
                },
                _ => {
                    r#"{"error":"Cloud not save document!"}"#
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
