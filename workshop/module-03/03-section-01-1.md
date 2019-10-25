#### Adding dependent packages
>[Cargo.toml](https://github.com/dsietz/rust-daas/blob/master/Cargo.toml)

---

First we need to include the dependent crates into the project. The crates we will need are `serde`, `serde_derive`, and `serde-json`. Add the following lines to the `[dependencies]` section in the **Cargo.toml** file  

```
serde ="1.0"
serde_derive = "1.0"
serde_json = "1.0.39"
```