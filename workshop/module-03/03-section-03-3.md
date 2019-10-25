#### Creating the module
>[broker.rs](https://github.com/dsietz/rust-daas/blob/master/src/broker.rs)

---

To create the module, create a new file named **_broker.rs_** in the **/src** directory.

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.

Add the `use` declarations at the top of the file.

```
use super::*;
use std::{thread};
use std::time::Duration;
use kafka::client::KafkaClient;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::{ErrorKind, KafkaCode};
```

##### Tests

First create the tests for this module as a nested module at the bottom of the file.

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_message() {
        match produce_message("Hello Kafka...".as_bytes(), "testTopic", vec!(KAFKA_BROKERS.to_string())) {
                Ok(_v) => {
                    assert!(true);
                },
                Err(e) => {
                    println!("Failed to send message to {}: {:?}", KAFKA_BROKERS.to_string(), e);
                    assert!(false);
                }
        }
    }
}
```

##### Code

Unlike in other dependency modules, (`daas` and `couchdb`) we will not be following an Object Oriented Design. The `kafka` crate that has been included is a robust library that supports the functionality we are interested in using. This module is simply to act as an abstraction layer.

```
pub fn produce_message<'a, 'b>(data: &'a [u8], topic: &'b str, brokers: Vec<String>) -> Result<(), kafka::error::ErrorKind> {
    let mut client = KafkaClient::new(brokers);

    let mut attempt = 0;
    loop {
        attempt += 1;
        let _ = client.load_metadata(&[topic])?;
        if client.topics().partitions(topic).map(|p| p.len()).unwrap_or(0) > 0 { // <-- HERE
            break;
        } else if attempt > 2 { // try up to 3 times
            // return some error
            return Err(ErrorKind::Kafka(KafkaCode::UnknownTopicOrPartition));
        }
        thread::sleep(Duration::from_secs(1));
    }

    let mut producer =
        Producer::from_client(client)
             .with_ack_timeout(Duration::from_secs(1))
             .with_required_acks(RequiredAcks::One)
             .create()?;

    producer.send(&Record{
        topic: topic,
        partition: -1,
        key: (),
        value: data,
    })?;

    Ok(())
}
```

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.