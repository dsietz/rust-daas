### Section V
>[rust-daas.postman_collection.json](https://github.com/dsietz/rust-daas/blob/master/tests/rust-daas.postman_collection.json)

---

A very powerful feature of the `actix-web` crate is the built in service testing that comes out-of-the-box. We utilized this feature in `web-service-tests.rs` as part of our integrated testing. Another more independent tool for service testing is [Postman](../../docs/reference-postman.md). 

You can [import](https://learning.getpostman.com/docs/postman/collections/data_formats/#importing-postman-data) the `rust-daas.postman_collection.json` file and perform the service tests.

On the command line, run the `cargo run --bin sourcing` command to start the service. (Or open a new command terminal and start the service using the executable in the target/debug directory).

```
PS C:\workspace\rust-daas> cargo run --bin sourcing
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target\debug\sourcing.exe`
```

Now you can open the `stage-data` POST request in Postman and monitor the logging on the command line of the running service.

>**Note** If you get an error message stating "Could not broker document", then make sure to go into the `lib.rs` file and check the value of the KAFKA_BROKERS variable. 

The response payload should be the following:

```
{
    "status": "OK"
}
```

> _TIP_ If you start up a consumer (e.g.: [Quick Start - Step 5](https://kafka.apache.org/quickstart#quickstart_consume)), you should see the brokered DaaS object. 
> 
```
[kafka@brokerserver kafka_2.12-2.3.0]# bin/kafka-console-consumer.sh --bootstrap-server mybroker:9092 --topic order-clothing-iStore --from-beginning
{"_id":"order|clothing|iStore|8003","_rev":"2-c295ee927aff853b50814447981173e0","source_name":"iStore","source_uid":8003,"category":"order","subcategory":"clothing","author":"istore_app","process_ind":false,"last_updated":1572021078,"data_obj":{"name":"high heals","status":"new"}}
```