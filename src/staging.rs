use rocket::{Rocket};
use rocket_contrib::json::{Json, JsonValue};
use super::*;

#[derive(Serialize, Deserialize)]
struct ObjData {
    data: JsonValue
}

fn build_db_doc(auth: &str, category: String, subcategory: String, source_name: String, source_uid: String, message: json::JsonValue) -> Result<json::JsonValue, String>{
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

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[post("/stage/<category>/<subcategory>/<source_name>/<source_uid>", format = "json", data = "<message>")]
fn stage(auth: HdrBase64, category: String, subcategory: String, source_name: String, source_uid: String, message: Json<ObjData>) -> JsonValue {
    //let dat = json::parse(message.data.as_str().unwrap()).unwrap();
    let doc = build_db_doc(&auth.0, category, subcategory, source_name, source_uid, message.data).unwrap();
    println!("DOCUMENT: {}", doc);
    json!({"status": "OK"})
}

pub fn service() -> Rocket {
    rocket::ignite().mount("/", routes![index, stage])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
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
        let db_doc = build_db_doc("Basic Zm9vOmJhcg==", "order".to_string(), "clothing".to_string(), "iStore".to_string(), "5000".to_string(), dat).unwrap();
        assert_eq!(db_doc, baseline);
    }
}
