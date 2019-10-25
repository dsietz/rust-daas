### Section III
>[processor.rs](https://github.com/dsietz/rust-daas/blob/master/src/processor.rs)

---

To create the module, create a new file named **_processor.rs_** in the **/src** directory.

We inherit most of our `use` declarations from `lib.rs` using the `use super::*;` statement, but there is the kafka consumer `use` to add.

```
use super::*;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, Message};
```

##### Tests

Add the following unit test to the bottom of the module.

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_consumer() {
        let _consumer = OrderStatusProcessor::new(KAFKA_BROKERS.to_string(), "test".to_string(), "my-group".to_string());
    }
}
```


##### Code

Because the processor executable is a stand-alone microservice, we will follow an Object Oriented Design. This means we first define the `OrderStatusProcessor` object.

```
#[derive(Debug)]
pub struct OrderStatusProcessor{
    pub consumer: Consumer,
    listen_ind: bool,
}
```

We then give the object functionality by using the `impl OrderStatusProcessor {...}` syntax. The constructor for the `OrderStatusProcessor` object is the `new()` function that returns a `OrderStatusProcessor` object.

```
impl OrderStatusProcessor {
    pub fn new(host: String,  topic: String, group_id: String) -> OrderStatusProcessor {
        OrderStatusProcessor{
            consumer: Consumer::from_hosts(vec!(host.to_owned()))
                        .with_topic(topic.to_owned())
                        .with_fallback_offset(FetchOffset::Earliest)
                        .with_group(group_id.to_owned())
                        .with_offset_storage(GroupOffsetStorage::Kafka)
                        .create()
                        .unwrap(),
            listen_ind: false,
        }
    }

    pub fn start_listening(&mut self, callback: fn(kafka::consumer::Message)) {
        self.listen_ind = true;

        while self.listen_ind {
            for messageset in self.consumer.poll().unwrap().iter() {
                for message in messageset.messages() {
                    callback(Message {
                        offset: message.offset,
                        key: message.key,
                        value: message.value,
                    });
                }
                match self.consumer.consume_messageset(messageset) {
                    Ok(_c) => {},
                    Err(err) => panic!("{}",err),
                }
            }
            self.consumer.commit_consumed().unwrap();
        }
    }

    pub fn stop_listening(&mut self) {
        self.listen_ind = false;
    }
}
```

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.