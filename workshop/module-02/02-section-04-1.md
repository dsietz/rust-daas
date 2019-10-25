#### Adding dependent packages
>[Cargo.toml](https://github.com/dsietz/rust-daas/blob/master/Cargo.toml)

---

We already have the binary file defined in the manifest file, but there are dependent packages that we will need to include in order to make it a RESTful service. In the **[dependencies]** section of the Cargo.toml file add the following packages.

```
hyper = "0.11.0"
actix-web = "0.7"
```