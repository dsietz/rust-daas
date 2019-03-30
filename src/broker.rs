use super::*;
use std::{thread, time};
use std::time::Duration;
use std::process::{Child, Command, Output, Stdio};
use kafka::client::KafkaClient;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::{ErrorKind, Error as Kafka, KafkaCode};


pub fn get_properties_path(prod: &str) ->String{
    match prod {
        "kafka" => format!("{}/config/server.properties", ZOOKEEPER_DIR),
        "zookeeper" => format!("{}/config/zookeeper.properties", ZOOKEEPER_DIR),
        _ => panic!("properties path for {} not supported", prod),
    }
}
pub fn get_run_cmd(prod: &str, start_stop: &str) -> String {
    let script = match cfg!(windows) {
            true => format!("windows/{}-server-{}.bat",prod, start_stop),
            false => format!("{}-server-{}.sh",prod, start_stop),
        };

    format!("{}/bin/{}", ZOOKEEPER_DIR, script)
}

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

pub fn run_cmd_with_properties(cmd: String, properties: String) -> Child {
    let child = Command::new(cmd)
        .args(&[properties])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("command failed to execute");
    
    child
}

pub fn run_cmd_without_properties(cmd: String) -> Child {
    let child = Command::new(cmd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("command failed to execute");
    
    child
}

pub fn start_kafka() -> Child {
    run_cmd_with_properties(get_run_cmd("kafka","start"), get_properties_path("kafka"))
}

pub fn stop_kafka() -> Child {
    run_cmd_without_properties(get_run_cmd("kafka","stop")) 
}

pub fn start_zookeeper() -> Child {
    run_cmd_with_properties(get_run_cmd("zookeeper","start"), get_properties_path("zookeeper"))
}

pub fn stop_zookeeper() -> Child {
    run_cmd_without_properties(get_run_cmd("zookeeper","stop"))  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_get_run_cmd_start_kafka() {
        let cmd = get_run_cmd("kafka","start");

        assert!(cmd.contains("kafka"));
        assert!(cmd.contains("start"));

        if cfg!(windows) {
            assert!(cmd.contains("windows"));
            assert!(cmd.contains(".bat"));
        } else if cfg!(unix) {
            assert!(cmd.contains(".sh"));
        }
    }

    #[ignore]
    #[test]
    fn test_get_run_cmd_stop_kafka() {
        let cmd = get_run_cmd("kafka","stop");

        assert!(cmd.contains("kafka"));
        assert!(cmd.contains("stop"));

        if cfg!(windows) {
            assert!(cmd.contains("windows"));
            assert!(cmd.contains(".bat"));
        } else if cfg!(unix) {
            assert!(cmd.contains(".sh"));
        }
    }

    #[ignore]
    #[test]
    fn test_get_run_cmd_start_zookeeper() {
        let cmd = get_run_cmd("zookeeper","start");

        assert!(cmd.contains("zookeeper"));
        assert!(cmd.contains("start"));

        if cfg!(windows) {
            assert!(cmd.contains("windows"));
            assert!(cmd.contains(".bat"));
        } else if cfg!(unix) {
            assert!(cmd.contains(".sh"));
        }
    }

    #[ignore]
    #[test]
    fn test_get_run_cmd_stop_zookeeper() {
        let cmd = get_run_cmd("zookeeper","stop");

        assert!(cmd.contains("zookeeper"));
        assert!(cmd.contains("stop"));

        if cfg!(windows) {
            assert!(cmd.contains("windows"));
            assert!(cmd.contains(".bat"));
        } else if cfg!(unix) {
            assert!(cmd.contains(".sh"));
        }
    }       

    #[ignore]
    #[test]
    fn test_zookeeper_start_stop() {
        let mut start = start_zookeeper();
        
        // sleep for a second
        let sec = time::Duration::from_secs(5);
        thread::sleep(sec);

        let stop = stop_zookeeper().wait().expect("failed to wait on child");
        assert!(stop.success());
    }

    #[ignore]
    #[test]
    fn test_kafka_start_stop() {
        let start = start_kafka();
        
        // sleep for a second
        let sec = time::Duration::from_secs(5);
        thread::sleep(sec);

        let stop = stop_kafka().wait().expect("failed to wait on child");
        assert!(stop.success());
    }    

    #[test]
    fn test_send_message() {
        assert!(true);
    }
}