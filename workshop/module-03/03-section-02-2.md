#### Updating the library
>[lib.rs](https://github.com/dsietz/rust-daas/blob/master/src/lib.rs)

---

We will also declare these dependencies in our share `src/lib.rs` library.

```
extern crate base64;
extern crate roadrunner;
```

To add the _daas_ module to our library, we declare it at the bottom of our `src/lib.rs` file, (after the daas module).

```
pub mod couchdb;
```