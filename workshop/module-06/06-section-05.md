### Section V
>[rust-daas.postman_collection.json](https://github.com/dsietz/rust-daas/blob/master/tests/rust-daas.postman_collection.json)

---

On the command line, run the `cargo run --bin reporting` command to start the service. (Or open a new command terminal and start the service using the executable in the target/debug directory).

```
PS C:\workspace\rust-daas> cargo run --bin reporting
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target\debug\reporting.exe`
```

Now you can open the `report data` POST request in Postman and monitor the logging on the command line of the running service.

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