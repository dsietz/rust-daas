use rocket::{Rocket};
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct ObjData {
    id: usize,
    data: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[post("/stage/<category>/<subcategory>/<source_name>/<source_uid>", format = "json", data = "<message>")]
fn stage(category: String, subcategory: String, source_name: String, source_uid: String, message: Json<ObjData>) -> JsonValue {
    json!({"status": "ok"})
}

pub fn service() -> Rocket {
    rocket::ignite().mount("/", routes![index, stage])
}