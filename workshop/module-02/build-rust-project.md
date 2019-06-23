## Module II - Building A Rust Project

In this module we will create a _Hello World_ RESTful service
1. Creating a project in Rust
2. Parts of a package: Cargo.toml, src/lib.rs vs. src/bin, modules
3. Test Driven Development
   - unit testing
   - integrated testing
4. Build a Hello World RESTful endpoint
5. Adding a middleware for logging

---

### Section I
Let first make sure the Rust is install correctly.

```
[user@localhost workspace]$ rustup -V
rustup 1.18.3 (435397f48 2019-05-22)
[user@localhost workspace]$ cargo -V
cargo 1.35.0 (6f3e9c367 2019-04-04)
[user@localhost workspace]$ rustup default stable
info: using existing install for 'stable-x86_64-pc-windows-msvc'
info: default toolchain set to 'stable-x86_64-pc-windows-msvc'

  stable-x86_64-pc-windows-msvc unchanged - rustc 1.35.0 (3c235d560 2019-05-20)

```

Create the _rust-daas_ package

```
[user@localhost workspace]$ cargo +stable new rust-daas --lib
     Created library `rust-daas` package
[user@localhost workspace]$ cd rust-daas
[user@localhost rust-daas]$ 
```

cargo has generated ...
```
.
|-- .git
|-- .gitignore
|-- src
     |-- lib.rs
|-- Cargo.toml  
```

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

### Section III

Although this package was created as a library, we can make it dual binary and library by doing the following:

1. Modifiy the Cargo.toml file to point to a binary during build time

```
[[bin]]
name = "hello_world"
path = "src/bin/hello-world.rs"
```

2. Create a new **bin** folder in the __src__ directory
3. Create a new file **hello-world.rs** in the __bin__ directory with the following code

```
pub fn main() {
    println!("Hello World");
}
```

4. Build and run the package 

```
[user@localhost rust-daas]$ cargo run
   Compiling rust-daas v0.1.0 (C:\workspace\rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 0.67s
     Running `target\debug\hello-world.exe`
Hello World
```

> There should now be an executable named **hello-world** in the /target/debug directory which you can execute directly.

```
[user@localhost rust-daas]$  cd .\target\debug\
[user@localhost debug]$ ./hello-world
Hello World
```