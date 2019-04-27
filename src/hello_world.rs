use super::*;
use actix_web::{App, HttpRequest, Responder};

pub fn get_service_root() -> String {
    format!("/hello/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/"
}

pub fn index(_req: &HttpRequest) -> impl Responder {
    "Hello World!".to_string()
}

pub fn service() -> App {
    let app = App::new()
                .middleware(Logger::default())
                .middleware(Logger::new("%a %{User-Agent}i"))
                .resource(
                    &get_service_path(), 
                    |r| r.get().f(index));
    app
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }

    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/hello/{}/", VER));
    }

    #[test]
    fn hello_response() {
        let resp = test::TestRequest::with_header("content-type", "text/plain")
            .run(&index)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
