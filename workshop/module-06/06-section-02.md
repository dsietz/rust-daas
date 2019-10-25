### Section II
>[lib.rs](https://github.com/dsietz/rust-daas/blob/master/src/lib.rs)

---

Since we didn't declare any new external dependencies in the `Cargo.toml` file, we don't have any new declarations in the `lib.rs` file. However, we do need to add the _data_services_ module to our library, we declare it at the bottom of our `src/lib.rs` file, (after the processor module).

```
pub mod data_service;
```