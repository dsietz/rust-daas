### Section II
The Rust package comes automatically setup with a unit test in the src/lib.rs file. You can build and test your package in one command.

```
[user@localhost rust-daas]$ cargo test
   Compiling rust-daas v0.1.0 (C:\workspace\rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 2.20s
     Running target\debug\deps\rust_daas-00b27fe51e56875b.exe

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests rust-daas

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

By default, the manifest file (Cargo.toml) has only the __[package]__ section which contains the meta data about the package. The __[dependencies]__ section is empty. Since we create this package as a library, the __[lib]__ section is "assumed" and the __src/lib.rs__ file is the default location and file name of the library module.

We will overwrite the defaults by adding the following lines to the __Cargo.toml__ file after the __[package]__ section.

```
[lib]
name = "daas"
path = "src/lib.rs"
```

try to rerun the build and test and notice that the library name changed from **rust-daas** to **daas**.

```
[user@localhost rust-daas]$ cargo test
   Compiling rust-daas v0.1.0 (C:\workspace\rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 0.75s
     Running target\debug\deps\daas-261fe2059b5b2d4b.exe

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests daas

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```