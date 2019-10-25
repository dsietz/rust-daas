#### Creating the module
>[hello_world.rs](https://github.com/dsietz/rust-daas/blob/master/src/hello_world.rs)

---

To create the module, create a new file named **_hello_world.rs_** in the **/src** directory. 

To begin, we will follow some basic TDD practices and build our tests first. 

>NOTE: This is not a TDD workshop, so we will ignore the complete practices and simply illustrate how it would be done.

At the bottom of the file, create an empty nested _testing_ module. This will be where we write our unit test for the hello_world module. The use ```super::*;``` line imports all the functionality and variables form the parent hello_world module.

```
#[cfg(test)]
mod tests {
    use super::*;
}
```

Our first test will be to return the service root. Add the following test in tests module below the ```use super::;``` line so it looks like the following.

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }
}
```

Following TDD practices, we now run our test and confirm that it will fail.

```
[user@localhost rust-daas]$ cargo test
   Compiling rust-daas v0.1.0 (C:\tmp\rust-daas)
error[E0425]: cannot find function `get_service_root` in this scope
  --> src\hello_world.rs:10:20
   |
10 |         assert_eq!(get_service_root(), format!("/hello/{}", VER));
   |                    ^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `test_get_service_root`

warning: unused imports: `App`, `HttpRequest`, `Responder`
 --> src\hello_world.rs:2:17
  |
2 | use actix_web::{App, HttpRequest, Responder};
  |                 ^^^  ^^^^^^^^^^^  ^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
error: Could not compile `rust-daas`.

To learn more, run the command again with --verbose.
```

To make the test pass, we will add the ```get_service_root()``` function to the module.

```
use super::*;

pub fn get_service_root() -> String {
    format!("/hello/{}", VER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }
}
```

If we rerun our test, it will now pass.

```
[user@localhost rust-daas]$ cargo test
   Compiling rust-daas v0.1.0 (C:\tmp\rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 1.60s
     Running target\debug\deps\daas-dafe2c98359dbcd2.exe

running 1 test
test hello_world::tests::test_get_service_root ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\hello_world-f32c48dd7c679e2e.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests daas

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

We will do the same for the ```get_service_path()``` function.

```
use super::*;

pub fn get_service_root() -> String {
    format!("/hello/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }

    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/hello/{}/", VER));
    }
}
```

Now that we have an understanding of how to write our tests, and then add the funcitonality to make them pass, we will move on to provide our service call.

Our test will be the following.

```
mod tests {
   use super::*;
   use actix_web::{http, test};

   #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }

    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/hello/{}/", VER));
    }

   #[test]
    fn hello_response() {
        let resp = test::TestRequest::with_header("content-type", "text/plain")
            .run(&index)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
```

In order to make it pass, we will need to import the web service modules, and provide a ```index()``` function.

```
use actix_web::{App, HttpRequest, Responder};

pub fn index(_req: &HttpRequest) -> impl Responder {
    "Hello World!".to_string()
}
```

The final file should look like the following.

```
use super::*;
use actix_web::{App, HttpRequest, Responder};

pub fn get_service_root() -> String {
    format!("/hello/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/"
}

pub fn index(_req: &HttpRequest) -> impl Responder {
    "Hello World!".to_string()
}

mod tests {
   use super::*;
   use actix_web::{http, test};

   #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }

    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/hello/{}/", VER));
    }

   #[test]
    fn hello_response() {
        let resp = test::TestRequest::with_header("content-type", "text/plain")
            .run(&index)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
```

Rerun the tests to make sure it all passes.

We now have one last step, which is to add a function that will provide the service object. This is will nor be covered by unit testing and is instead covered by integrated testing.

To create integrated tests, first create a new file named **_web-service-tests.rs_** in a new directory named **_tests_** in the root path (same level as the **src** directory). Cargo will automatically parse the **_tests_** directory and run any tests that are located in any files located here.

```
.
|-- .git
|-- .gitignore
|-- src
     |-- bin
          |-- hello-world.rs
     |-- hello_world.rs
     |-- lib.rs
|-- tests
     | -- web-service-tests.rs
|-- Cargo.toml  
```

In order to execute our service test, we will first need to include the `bytes` library to our project. We do this by adding the line `bytes = "0.4"` in the `[dependencies]` section of the **_Cargo.toml_** file.

```
[dependencies]
hyper = "0.11.0"
actix-web = "0.7"
bytes = "0.4"
```

Once the library has been included in the Manifest, we define which libraries are required in the **_web-service-tests_** module by adding the following lines at the top of the **_web-service-tests.rs_** file.

```
extern crate actix_web;
extern crate bytes;
```

The `extern` declarations specify the dependent crates (or libraries) that will be used in the **_web-service-tests_** module. 

We then declare the bindings (or shortcuts) to a resources that will be used in the **_web-service-tests_** module. This is done by adding the following lines below the `extern` crate declarations.

```
use daas::hello_world;
use actix_web::*;
use http::header;
use bytes::Bytes;
```

Now we can add the code for our Hello World service test, which is added below the `use` declarations.

```
#[test]
fn test_hello_world_ok() {
    let mut srv =actix_web::test::TestServer::new(|app| app.handler(hello_world::index));
    let request = srv.get().uri(srv.url("/").as_str()).finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    // read response
    let bytes = srv.execute(response.body()).unwrap();
    assert_eq!(bytes, Bytes::from_static("Hello World!".as_ref()));
}
```

At this point the **_web-service-tests.rs_** file should look like this:

```
extern crate actix_web;
extern crate bytes;

use daas::hello_world;
use actix_web::*;
use http::header;
use bytes::Bytes;

#[test]
fn test_hello_world_ok() {
    let mut srv =actix_web::test::TestServer::new(|app| app.handler(hello_world::index));
    let request = srv.get().uri(srv.url("/").as_str()).finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    // read response
    let bytes = srv.execute(response.body()).unwrap();
    assert_eq!(bytes, Bytes::from_static("Hello World!".as_ref()));
}
```

Try running your test with the `cargo test` command. There should now be a line in the results referencing that the `web_service_tests` has run.

```

     Running target\debug\deps\web_service_tests-664800ae8a37eeb0.exe

running 1 test
test test_hello_world_ok ... ok
```

