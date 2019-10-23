#### Creating the executable

Once we have created our service and all the tests have passed, we are ready to build out the executable and run it.

We start by first adding the `log` and `env_logger` crates to the Cargo.toml manifest.

```
[dependencies]
log = "0.4.0"
env_logger = "0.6.1"
hyper = "0.11.0"
actix-web = "0.7"
bytes = "0.4"
```

We place the `extern crate` declarations and `use` declarations for these crates at the top of our **_lib.rs_** file (so that they are ben shared in the project).

```
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate actix_web;

use actix_web::middleware::Logger;

static VER: &str = "v1";

pub mod hello_world;
```

Now that we have the dependent crates included and declared, we add a new method named `service` after the index() method in the **_hello_world.rs_** file in the /src directory to provide a runtime application that references the `index90` method for the defined resource path referenced by the `get_service_path()` method.

```
pub fn service() -> App {
    let app = App::new()
                .middleware(Logger::default())
                .middleware(Logger::new("%a %{User-Agent}i"))
                .resource(
                    &get_service_path(), 
                    |r| r.get().f(index));
    app
}
```

Make sure all your tests are still passing by using the `cargo test` command.

---

We are now ready to start the RESTful service. There are 2 ways to start the service.

1. Running using `cargo run` command

```
PS C:\workspace\rust-daas> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target\debug\hello_world.exe`
```

Open your browser and navigate to the URL: [http://localhost:7999/hello/v1/](http://localhost:7999/hello/v1/). You should see the message `Hello World!`
On the command line, you will notice that the calls are being logged and printed to the consule.

```
[2019-10-23T14:40:08Z INFO  actix_web::middleware::logger] 127.0.0.1:65211 Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120 Safari/537.36
[2019-10-23T14:40:08Z INFO  actix_web::middleware::logger] 127.0.0.1:65211 "GET /hello/v1/ HTTP/1.1" 200 12 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120 Safari/537.36" 0.000000
```

To stop the service, use `ctrl` + `c`.

2. Running using the executable.

```
PS C:\workspace\rust-daas> cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
```

Whenever you use the `cargo build` command, it places the created executable in the target/debug directory with the same name that was defined in the Cargo.toml manifest, (e.g.: C:\workspace\rust-daas\target\debug\hello_world.exe)

Since it is an executable, simple run the executable from the command terminal, and make the same URL call from the browser.

```
C:\workspace\demo\rust-daas\target\debug>hello_world.exe
[2019-10-23T14:49:19Z INFO  actix_web::middleware::logger] 127.0.0.1:65360 Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120 Safari/537.36
[2019-10-23T14:49:19Z INFO  actix_web::middleware::logger] 127.0.0.1:65360 "GET /hello/v1/ HTTP/1.1" 200 12 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120 Safari/537.36" 0.000948
```


   

