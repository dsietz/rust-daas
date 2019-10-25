### Section III
>[data_service.rs](https://github.com/dsietz/rust-daas/blob/master/src/data_service.rs)

---

To create the module, create a new file named **_data_service.rs_** in the **/src** directory.

Since we will be using the `actix-web` and `couchdb` functionality, we need to declare our `use` statement at the top of the file.

```
use super::*;
use actix_web::{App, http, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use super::couchdb::{CouchDB};
use std::thread;
```

##### Tests

Add the following unit test to the bottom of the module to cover the basic funcitonality.

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/data/{}", VER));
    }
}

```


##### Code

We start by defining our modular variables.

```
static DB_NAME: &str = "consuming";
static DB_USER: &str = "admin";
static DB_PSWRD: &str = "password";
```

>**IMPORTANT** You will need to create a `consuming` database in CouchDB 

And then our generic supporting functions that provide the resource path.

```
pub fn get_service_root() -> String {
    format!("/data/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/history/status"
}
```

Next, we define the supportive function that the serivce will call.

```
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
```

Lastly, we define the funciton that will provide the `App` object to the executable.

```
pub fn service() -> App {
    let app = App::new()
                .middleware(Logger::default())
                .middleware(Logger::new("%a %{User-Agent}i"))
                .resource(
                    &get_service_path(),
                    |r| r.get().with(status_history));
    app
}

```

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.