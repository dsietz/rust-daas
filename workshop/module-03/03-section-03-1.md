#### Adding dependent packages
>[Cargo.toml](https://github.com/dsietz/rust-daas/blob/master/Cargo.toml)

---

First we need to include the dependent crates into the project. The crate we will need is `kafka`. However, we will need to define the dependent crate a bit differently than our other definitions because we will specify which features from the `kafka` crate we need to include. To define sepcifics on the dependent crate, add a new dependencies section labeled `[dependencies.kafka]` after the `[dependencies]` section in the **Cargo.toml** file.  

```
[dependencies.kafka]
version = "0.7.0"
default-features = false
features = ["snappy","gzip"]
```