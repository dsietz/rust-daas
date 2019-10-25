#### Updating the library
>[lib.rs](https://github.com/dsietz/rust-daas/blob/master/src/lib.rs)

---

We will now need to define the `kafka` crate dependency in our share `src/lib.rs` library.

```
extern crate kafka;
```

Next, let's make our life easier by defining soem shared variables that we will be using throughout the library when interacting with the Kafka broker.
Just after the line `static VER: &str = "v1";` add the following statements:

```
pub static DELIMITER: &str = "-";
pub static KAFKA_BROKERS: &str = "localhost:9092";
static ZOOKEEPER_DIR: &str = "C:/kafka/kafka_2.12-2.2.0";
```


To add the _broker_ module to our library, we declare it at the bottom of our `src/lib.rs` file, (after the couchdb module).

```
pub mod broker;
```