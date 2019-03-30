use super::*;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

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

    pub fn start_listening(&mut self) {
        self.listen_ind = true;

        while self.listen_ind {
            for ms in self.consumer.poll().unwrap().iter() {
                for m in ms.messages() {
                    println!("{:?}", m);
                }
                self.consumer.consume_messageset(ms);
            }
            self.consumer.commit_consumed().unwrap();
        }
    }

    pub fn stop_listening(&mut self) {
        self.listen_ind = false;
    }
}


/*
let mut consumer =
   Consumer::from_hosts(vec!("localhost:9092".to_owned()))
      .with_topic_partitions("test".to_owned(), &[0, 1])
      .with_fallback_offset(FetchOffset::Earliest)
      .with_group("my-group".to_owned())
      .with_offset_storage(GroupOffsetStorage::Kafka)
      .create()
      .unwrap();
loop {
  for ms in consumer.poll().unwrap().iter() {
    for m in ms.messages() {
      println!("{:?}", m);
    }
    consumer.consume_messageset(ms);
  }
  consumer.commit_consumed().unwrap();
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_consumer() {
        let consumer = OrderStatusProcessor::new("localhost:9092".to_string(), "test".to_string(), "my-group".to_string());
    }
}