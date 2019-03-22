use super::*;
use actix_web::{App, http, HttpMessage, HttpRequest, Responder, Result};
use super::daas::DaaSDoc;
use super::couchdb::{CouchDB};
use std::thread;

pub fn get_service_root() -> String {
    format!("/stage/{}", VER)
}

pub fn index(req: &HttpRequest) -> impl Responder {
    "Hello World!".to_string()
}

//https://docs.rs/actix-web-httpauth/0.1.0/actix_web_httpauth/headers/authorization/struct.Authorization.html
pub fn stage(req: &HttpRequest) -> Result<String> {
    let cat: String = req.match_info().query("category")?;
    let subcat: String = req.match_info().query("subcategory")?;
    let srcnme: String = req.match_info().query("source_name")?;
    let srcuid: usize = req.match_info().query("source_uid")?;
    println!("{:?}", req.payload());
    
    let data = json!({
        "quantitiy":1,
        "status": "new"
    });
    
    let doc = DaaSDoc::new(srcnme, srcuid, cat, subcat, data);
    let couch = CouchDB::new("admin".to_string(), "password".to_string());

    let save = thread::spawn(move || {
            match couch.upsert_doc("test".to_string(),doc) {
                Ok(rslt) => Ok("{\"status\":\"OK\"}".to_string()),
                _ => Ok("{\"status\":\"ERROR\"}".to_string()),
            }
        });
    save.join().unwrap()
}

pub fn service() -> App {
    let app = App::new()
                .resource(
                    "/", 
                    |r| r.method(http::Method::GET).f(index))
                .resource(
                    &(get_service_root() + "/{category}/{subcategory}/{source_name}/{source_uid}"),
                    |r| r.method(http::Method::POST).f(stage));
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
