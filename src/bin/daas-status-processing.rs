use daas::processor::{OrderStatusProcessor};
use daas::{DELIMITER};
use serde_json::{Value};

/// globals
static DB_NAME: &str = "provisioning";
static DB_USER: &str = "admin";
static DB_PSWRD: &str = "password";

pub fn main() {
    let topic = format!("{}{}{}{}{}", "order", DELIMITER, "clothing", DELIMITER, "iStore");

    let mut status_processor = OrderStatusProcessor::new("localhost:9092".to_string(), topic, "my-group".to_string());
    status_processor.start_listening(|msg|{
            let value = String::from_utf8(msg.value.to_vec()).unwrap();
            let order: Value = serde_json::from_str(&value).unwrap();
            println!("{:?}", order);
        });
}