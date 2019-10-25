### Section III
>[sourcing.rs](https://github.com/dsietz/rust-daas/blob/master/src/sourcing.rs)

---

To create the module, create a new file named **_sourcing.rs_** in the **/src** directory.

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.

Add the `use` declarations at the top of the file.

```
use super::*;
use actix_web::{App, http, HttpRequest, HttpResponse, Path};
use actix_web_httpauth::extractors::basic::BasicAuth;
use super::daas::DaaSDoc;
use super::couchdb::{CouchDB};
use std::thread;
```

We will also define some global variables for the module.

> Note: You will need to create the `source` database before running the tests.

```
static DB_NAME: &str = "sourcing";
static DB_USER: &str = "admin";
static DB_PSWRD: &str = "password";
```

##### Tests

Add the following unit test to the bottom of the module.

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/stage/{}", VER));
    }

    #[test]
    fn test_get_serive_path() {
        assert_eq!(get_service_path(), format!("/stage/{}/{}", VER, "{category}/{subcategory}/{source_name}/{source_uid}"));
    }
}
```


##### Code

In order to support the RESTful service, we first need to define two structures (objects) that will represent parts of the incoming requests.

1. Resource parameters (named _Info_)
2. Payload (named _Bdy_)

After the global variables, add the following lines of code.

```
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
```

Since th `actix-web` already provide OOD (e.g.: _App::new()_) for our RESTful service, we will simply supply the supporting functions in the module that this _App_ objective will reference.

Let's first add the generic functions that provides the resource pathes. These will go after the structures that we defined above.

```
pub fn get_service_root() -> String {
    format!("/stage/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/{category}/{subcategory}/{source_name}/{source_uid}"
}
```

Our next section of code add the _data processing_ fucntionality that will be called when a request is mand to stage the data.

```
fn process_data(couch: CouchDB, id: String, topic: String) -> Result<bool, String>{
    match couch.get_doc_by_id(DB_NAME.to_string(), id) {
        Ok(mut doc) => {
            match broker::produce_message(&doc.serialize().as_bytes(), &topic.clone(), vec!(KAFKA_BROKERS.to_string())) {
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
```

In place of an _index()_ funciton that is called then the resoruce service is requested, we will name the funciton `source()` for clarificaiton and readability.

Notice that the function requires parameters for:
+ BasicAuth
+ the `Info` structure found in the request `Path<>` that represents the resource parameters
+ the payload that will follow the `Bdy` structure 
+ and the request itself - `HttpRequest`

```
pub fn source(auth: BasicAuth, params: Path<Info>, body: String, _req: HttpRequest) -> HttpResponse {
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
```

Now that we have all the supporting functions for the actix-web `App`, we provide a wrapper funciton that returns the `App` for the executable. This way, the code for the RESTful service executable doesn't have to have knoweldge of the service itself, (e.g.: Logging, resoruce path, function to call, etc.) The control of the service remains in the library and not at the point of implementation.  

```
pub fn service() -> App {
    let app = App::new()
                .middleware(Logger::default())
                .middleware(Logger::new("%a %{User-Agent}i"))
                .resource(
                    &get_service_path(),
                    |r| r.post().with(source));
    app
}
```

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.