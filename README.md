# rust-daas

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Coverage Status](https://coveralls.io/repos/github/dsietz/rust-daas/badge.svg?branch=master)](https://coveralls.io/github/dsietz/rust-daas?branch=master)
[![Docs.rs](https://docs.rs/rust-daas/badge.svg)](https://docs.rs/rust-daas)

Linux: [![Build Status](https://travis-ci.org/dsietz/rust-daas.svg?branch=master)](https://travis-ci.org/dsietz/rust-daas)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/5w1x4q7b8g29ijvi?svg=true)](https://ci.appveyor.com/project/dsietz/rust-daas/branch/master)

---

## Hands-On experience with Data as a Service (DaaS)

Should Information Management systems apply the services architecture? Many data provisioning and BI systems are monolithic, tightly coupled, difficult to scale, and stumble when it comes to delivering MVP in a timely manner.

Data as a Service delivers MVP of real-time data management, while avoiding many of the anit-patterns that traditional data provisioning and BI systems portray. Unlike traditional BI tooling, building out a Data as a Service system doesn't require high up-front costs and the welding of multiple products.

Get hands-on experience learning how the Rust language, a Kafka broker, and CouchDB cluster can be used to build out a DaaS system that delivers faster and more scalable solutions to your customer.

In this workshop we will walk-through and implement the key components of the Data as a Service architecture pattern by building out a simple real-time event driven online report.


>In this workshop you will learn how to 
>
>+ create RESTful services in Rust
>+ broker the events using Kafka
>+ provide data analytics as a service

---

__IMPORTANT:__ Participants need to ...

+ Bring their own development devices
+ Have installed Rust Toolchain, Kafak, and CouchDB prior to the workshop
+ Have internet connect during the workshop

---

## Developer Slice Setup
- [Rust Language](./docs/reference-rust.md)
- [CouchDB](./docs/reference-couchdb.md)
- [Kafak](./docs/reference-kafka.md)

---

## Workshop Material
+ [Workshop Manual](https://app.gitbook.com/@davidsietz/s/workspace/docs)
+ [Handbook](./docs/handbook.md)
+ [Rust related references](./docs/reference-material.md)
