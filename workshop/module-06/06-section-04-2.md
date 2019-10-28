### Section IV
>[Cargo.toml](https://github.com/dsietz/rust-daas/blob/master/Cargo.toml)

>[daas-reporting.rs](https://github.com/dsietz/rust-daas/blob/master/src/bin/daas-reporting.rs)

---

We are now ready to write the executable. To do this, we will working with the following files:

+ Cargo.toml (manifest)
+ src/bin/daas-reporting.rs (executable) 

##### Tests

>To save time on this Workshop, we will skip the create of integrated service tests for this RESTful service.

##### Code

**Declaring the Executable**

In the `Cargo.toml` file, after the that last `[[bin]]` statement for _status_processor_, add the following declaration.

```
[[bin]]
name = "reporting"
path = "src/bin/daas-reporting.rs"
```

This will tell Cargo that there is a binary file to be compiled and to name the exeutable **reporting**.

**Coding the Executable** 

Let's begin by creating a file named `daas-reporting.rs` in the `src/bin/` directory.

At the top of the file, we will start be declare the dependent crate with macros and the `use` statements.

```
use daas::data_service;
use actix_web::{server};
```

Lastly, we define the `main()` function that will be called by the executable.

```
pub fn main() {    
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    server::new( || {data_service::service()})
    .bind("127.0.0.1:8001")
    .expect("Can not bind to port 8001")
    .run();
}
``` 