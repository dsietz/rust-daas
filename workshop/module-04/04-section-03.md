### Section II
>[sourcing.rs](https://github.com/dsietz/rust-daas/blob/master/src/sourcing.rs)

To create the module, create a new file named **_sourcing.rs_** in the **/src** directory.

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.

Add the `use` declarations at the top of the file.

```
use super::*;
use actix_web::{App, http, HttpRequest, HttpResponse, Path};
use actix_web_httpauth::extractors::basic::BasicAuth;
use super::daas::DaaSDoc;
use super::couchdb::{CouchDB};
use std::thread;
```

We will also define some global variables for the module.

```
static DB_NAME: &str = "sourcing";
static DB_USER: &str = "admin";
static DB_PSWRD: &str = "password";
```

##### Tests

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/stage/{}", VER));
    }

    #[test]
    fn test_get_serive_path() {
        assert_eq!(get_service_path(), format!("/stage/{}/{}", VER, "{category}/{subcategory}/{source_name}/{source_uid}"));
    }
}
```


> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.