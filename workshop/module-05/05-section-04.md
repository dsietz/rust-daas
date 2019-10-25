### Section IV
>[Cargo.toml](https://github.com/dsietz/rust-daas/blob/master/Cargo.toml)

>[daas-status-processing.rs](https://github.com/dsietz/rust-daas/blob/master/src/bin/daas-status-processing.rs)

---

We are now ready to write the executable. To do this, we will working with the following files:

+ Cargo.toml (manifest)
+ src/bin/daas-status-processing.rs (executable) 

##### Tests

 
##### Code

**Declaring the Executable**

In the `Cargo.toml` file, after the that last `[[bin]]` statement for _sourcing_, add the following declaration.

```
[[bin]]
name = "status_processor"
path = "src/bin/daas-status-processing.rs"
```

This will tell Cargo that there is a binary file to be compiled and to name the exeutable **status_processor**.

**Coding the Executable** 

Let's begin by creating a file named `daas-status-processing.rs` in the `src/bin/` directory.

At the top of the file, we will start be declare the dependent crate with macros and the `use` statements.

```
#[macro_use] extern crate serde_derive;

use daas::{DELIMITER};
use daas::daas::{DaaSDoc};
use daas::processor::{OrderStatusProcessor};
use daas::couchdb::{CouchDB};
use serde_json::{json, Value};
use daas::KAFKA_BROKERS;
```

We then define the module variables for this executable.

```
static DB_NAME: &str = "provisioning";
static DB_USER: &str = "admin";
static DB_PSWRD: &str = "password";
```

To make our work easier, we will utilize a `StatusRecord` object to handle the status data.

```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusRecord{
    pub name: String,
    pub timestamp: u64,
}
```

Next we need to provide the supportive function to handle updating the status history. 

```
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
```

Now, we are ready to writ ethe `main()` function that will be called when the executable starts.

```
pub fn main() {
    let topic = format!("{}{}{}{}{}", "order", DELIMITER, "clothing", DELIMITER, "iStore");

    let mut status_processor = OrderStatusProcessor::new(KAFKA_BROKERS.to_string(), topic, "my-group".to_string());
    status_processor.start_listening(|msg|{
            let value = String::from_utf8(msg.value.to_vec()).unwrap();
            let order: Value = serde_json::from_str(&value).unwrap();

            match update_order_status_history(order.clone()) {
                Ok(_v) => println!("processed order status history for {} order number {}.", order["source_name"], order["source_uid"]),
                Err(err) => println!("Couldn't process order status history for {} order number {} because of {}!", order["source_name"], order["source_uid"], err),
            }
        });
}
```


> **IMPORTANT** 
> + You will need to create a database named `provisioning` in CouchDB in order for this service to work.
> + You will need to create a topic named `test` in the Kafka broker in order for this unit tests to pass.


>Try to rerun the `cargo test` command, and ensure that all the test pass.

##### Running the Microsservice

On the command line, run the `cargo run --bin status_processor` command to start the service. (Or open a new command terminal and start the service using the executable in the target/debug directory).

```
PS C:\workspace\rust-daas> cargo run --bin status_processor
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target\debug\status_processor.exe`
processed order status history for "iStore" order number 8003.
processed order status history for "iStore" order number 8003.
processed order status history for "iStore" order number 8004.
``

Look in the CouchDB `provisioning` database and there should be documents for the aggregates by order number.

_Example_

```
{
  "_id": "history|status|iStore|8003",
  "_rev": "3-9b8f57b145ddfea0e3fec0147a6cb835",
  "source_name": "iStore",
  "source_uid": 8003,
  "category": "history",
  "subcategory": "status",
  "author": "istore_app",
  "process_ind": false,
  "last_updated": 1572027379,
  "data_obj": {
    "order_number": 8003,
    "order_status": [
      {
        "name": "new",
        "timestamp": 1572021078
      },
      {
        "name": "new",
        "timestamp": 1572021537
      },
      {
        "name": "acknowledge",
        "timestamp": 1572027549
      }
    ],
    "order_type": "clothing",
    "store_name": "iStore",
    "timestamp": 1572021078
  }
}
```