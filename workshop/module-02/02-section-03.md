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
     Running `target\debug\hello_world.exe`
Hello World
```

> There should now be an executable named **hello_world** in the /target/debug directory which you can execute directly.

```
[user@localhost rust-daas]$  cd .\target\debug\
[user@localhost debug]$ ./hello_world
Hello World
```