#### Adding dependent packages
>[Cargo.toml](https://github.com/dsietz/rust-daas/blob/master/Cargo.toml)

---

First we need to include the dependent crates into the project. The crates we will need are `roadrunner`, `base64`, `tokio-core`, and `rand`. Add the following lines to the `[dependencies]` section in the **Cargo.toml** file  

```
base64 = "0.10.1"
roadrunner = "0.1.1"
tokio-core = "0.1.17"
rand = "0.6.5"
```