use super::*;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, Message};

#[derive(Debug)]
pub struct OrderStatusProcessor{
    pub consumer: Consumer,
    listen_ind: bool,
}

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
                    callback(Message{
                        offset: message.offset,
                        key: message.key,
                        value: message.value,
                    });
                }
                self.consumer.consume_messageset(messageset);
            }
            self.consumer.commit_consumed().unwrap();
        }
    }

    pub fn stop_listening(&mut self) {
        self.listen_ind = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_consumer() {
        let consumer = OrderStatusProcessor::new("localhost:9092".to_string(), "test".to_string(), "my-group".to_string());
    }
}