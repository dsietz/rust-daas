### Section I
>[Cargo.toml](https://github.com/dsietz/rust-daas/blob/master/Cargo.toml)

---

In Module II, we build out a RESTful service that responded with "Hello World!" whenever a GET call was made to the service. The coding of this service was comprised of 3 basic parts.

1. Declarations of dependent, (e.g.: Crates.toml, `extern crate` and `use`)
2. Constructing the module, (e.g.: src/hello_world.rs)
3. Constructing the executable service, (e.g.: src/bin/hello-world.rs)

We will follow the same pattern for building out this RESTful service for sourcing data to our reporting system.

---

Let beginning with declaring our dependencies. Because we will be implementing Basic Authentication, we will need to include the `actix-web-httpauth` crate that extends the `actix-web` package. This should be declared in the `[dependencies]` section of the **Cargo.toml** file

```
actix-web-httpauth = "0.1.0"
```

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.