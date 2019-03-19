use super::*;
use actix_web::{App, http, HttpRequest, Responder, Result};

/*
#[derive(Deserialize)]
pub struct Params {
    category: String,
    subcategory: String,
    source_name: String,
    source_uid: usize,
}
*/
fn build_db_doc(auth: &str, category: String, subcategory: String, source_name: String, source_uid: usize, message: json::JsonValue) -> Result<json::JsonValue, String>{
    match get_author(auth) {
        Some(auth) => {
            let doc = object!{
                "_id" => "1|2|3",
                "author" => auth,
                "source_name" => source_name,
                "source_uid" => source_uid,
                "category" => category,
                "subcategory" => subcategory,
                "last_modified" => get_unix_now(),
                "data_object" => message
            };
            
            Ok(doc)
        },
        None => Err("Warning: Could not extract author from Authorization header.".to_string())
    }
}

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
    let srcuid: String = req.match_info().query("source_uid")?;

    println!("PARAMETERS: {}, {}, {}, {} ...", cat, subcat, srcnme, srcuid);
    Ok("{\"status\":\"OK\"}".to_string())
}

pub fn service() -> App {
    let app = App::new()
                .resource(
                    "/", 
                    |r| r.method(http::Method::GET).f(index))
                .resource(
                    &(get_service_root() + "/{category}/{subcategory}/{source_name}/{source_uid}"),
                    |r| r.method(http::Method::GET).f(stage));
    app
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_db_doc_ok() {
        let baseline = object!{
                            "_id" => "1|2|3",
                            "author" => "foo",
                            "source_name" => "iStore",
                            "source_uid" => 5000,
                            "category" => "order",
                            "subcategory" => "clothing",
                            "last_modified" => 1234567890,
                            "data_object" => object!{
                                "name" => "leather coat",
                                "status" => "new"
                            }
                        };

        //println!("BASELINE: {}", build_db_doc("Basic Zm9vOmJhcg==").unwrap());
        let dat = json::parse(r#"{"name": "leather coat", "status":"new"}"#).unwrap();
        let db_doc = build_db_doc("Basic Zm9vOmJhcg==", "order".to_string(), "clothing".to_string(), "iStore".to_string(), 5000, dat).unwrap();
        assert_eq!(db_doc[0], baseline[0]); // _id 
        assert_eq!(db_doc[1], baseline[1]); // author
        assert_eq!(db_doc[2], baseline[2]); // source_name
        assert_eq!(db_doc[3], baseline[3]); // source_uuid
        assert_eq!(db_doc[4], baseline[4]); // category
        assert_eq!(db_doc[5], baseline[5]); // subcategory
        assert_eq!(db_doc[7], baseline[7]); // data_object
    }

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/stage/{}", VER));
    }
}
