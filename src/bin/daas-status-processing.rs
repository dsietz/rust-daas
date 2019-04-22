#[macro_use] extern crate serde_derive;

use daas::{DELIMITER};
use daas::daas::{DaaSDoc};
use daas::processor::{OrderStatusProcessor};
use daas::couchdb::{CouchDB};
use serde_json::{json, Value};

/// globals
static DB_NAME: &str = "provisioning";
static DB_USER: &str = "admin";
static DB_PSWRD: &str = "password";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusRecord{
    pub name: String,
    pub timestamp: u64,
}

fn update_order_status_history(order: Value) -> Result<bool, String>{
    let category = "history".to_string();
    let subcategory = "status".to_string();
    let id = daas::daas::make_id(category.clone(), subcategory.clone(), order["source_name"].as_str().unwrap().to_string(), order["source_uid"].as_u64().unwrap() as usize); 
    let couch = CouchDB::new(DB_USER.to_string(), DB_PSWRD.to_string());
    let status = StatusRecord{
            name: order["data_obj"]["status"].as_str().unwrap().to_string(),
            timestamp: order["last_updated"].as_u64().unwrap(),
        };

    match couch.get_doc_by_id(DB_NAME.to_string(), id) {
        Ok(mut doc) => {
            //update the status history of an existing document
            let status_history = doc.data_obj_as_ref()["order_status"].as_array_mut().unwrap();
            
            status_history.push(json!(status));
            couch.upsert_doc(DB_NAME.to_string(), doc).unwrap();

            Ok(true)       
        },
        _ => {
            //create a new status history document
            let hist = json!({
                "order_number": order["source_uid"],
                "store_name": order["source_name"],
                "order_type": order["subcategory"],
                "timestamp": order["last_updated"],
                "order_status": [status],
            });

            let uid = order["source_uid"].as_u64().unwrap() as usize;
            let src_nme = order["source_name"].as_str().unwrap().to_string();
            let athr = order["author"].as_str().unwrap().to_string();
            let doc = DaaSDoc::new(src_nme, uid, category, subcategory, athr, hist);

            couch.upsert_doc(DB_NAME.to_string(), doc).unwrap();

            Ok(true)
        }
    }
}

pub fn main() {
    let topic = format!("{}{}{}{}{}", "order", DELIMITER, "clothing", DELIMITER, "iStore");

    let mut status_processor = OrderStatusProcessor::new("localhost:9092".to_string(), topic, "my-group".to_string());
    status_processor.start_listening(|msg|{
            let value = String::from_utf8(msg.value.to_vec()).unwrap();
            let order: Value = serde_json::from_str(&value).unwrap();

            match update_order_status_history(order.clone()) {
                Ok(_v) => println!("processed order status history for {} order number {}.", order["source_name"], order["source_uid"]),
                Err(err) => println!("Couldn't process order status history for {} order number {} because of {}!", order["source_name"], order["source_uid"], err),
            }
        });
}