use super::*;
use actix_web::{App, http, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use super::couchdb::{CouchDB};
use std::thread;

/// globals
static DB_NAME: &str = "consuming";
static DB_USER: &str = "admin";
static DB_PSWRD: &str = "password";

pub fn get_service_root() -> String {
    format!("/data/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/history/status"
}

pub fn status_history(_auth: BasicAuth, _req: HttpRequest) -> HttpResponse {
    let couch = CouchDB::new(DB_USER.to_string(), DB_PSWRD.to_string());
    let reply = thread::spawn(move || {
            match couch.query_view(DB_NAME.to_string(), "_design/history/_view/status-duration?reduce=true&group=true".to_string()) {
                Ok(results) => {
                    results.clone()
                },
                _ => {
                    r#"{"error": "Could not find the document!"}"#.to_string()
                },
            }
        });

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(reply.join().unwrap())  
}

pub fn service() -> App {
    let app = App::new()
                .middleware(Logger::default())
                .middleware(Logger::new("%a %{User-Agent}i"))
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
