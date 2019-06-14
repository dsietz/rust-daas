# Data as a Service Workshop

## Objective
> The goal of this workshop is to provide participants with the hands-on experience to build out a DaaS architecture pattern.

In this workshop you will learn: 
+ Overview of the DaaS pattern
+ Overview of the Rust language
+ How to create RESTful services in Rust
+ How to broker the events using Kafka
+ How to provide data analytics as a service

---
## Module I
+ [_Overview of the DaaS Pattern_](modeule-01/daas-pattern.md#daas-pattern)


## Module I
_Creating a Hello World RESTful service_
1. Creating a project in Rust
2. Parts of a package: Cargo.toml, src/lib.rs vs. src/bin, modules
3. Test Driven Development
   - unit testing
   - integrated testing
4. Build a Hello World RESTful endpoint
5. Adding a middleware for logging

## Module II
_Creating a RESTful service to source event data_
1. Build a data sourcing RESTful endpoint
2. Making a parameterized resource path
3. Adding authentication

## Module III
_Creating a DaaS module_
1. make_id()
2. DaaSDoc struc
3. DaaSDocNoRev struc
4. implement DaaSDoc

## Module IV
_Creating a CouchDB module_
1. CouchDB struc
2. implement CouchDB
   
## Module V
_Creating a Kafka broker module_
1. get_properties_path()
2. get_run_cmd()
3. produce_message()
4. run_cmd_with_properties()
5. run_cmd_without_properties()
6. start_zookeeper() / stop_zookeeper()
7. start_kafka() / stop_kafka()

## Module VI
_Creating a data provisioning processor for managing order status metrics_
1. OrderStatusProcessor struc
2. implement OrderStatusProcessor
3. replace stub for sourcing::process_data()

## Module II
_Creating a RESTful service to service up order status metrics_
1. CouchDB view 
   ![Status Duraiton View](_view-status-duration.png)
2. Build a data service RESTful endpoint for metrics