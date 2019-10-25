#### Updating the library
>[lib.rs](https://github.com/dsietz/rust-daas/blob/master/src/lib.rs)

---

The lib.rs file is our centralized library file. This file is where globally shared functions, constants and variables are located. It is also where we include modules that make up the library.

**Remove** the test module section of code at the bottom of the file. This is where your integrated testing (module dependencies) would be tested to ensure the library if working as expected. However, we will be ignoring this level of testing for the purpose of this workshop.

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

At the very top of the file, add the external web service crate (package)

```
extern crate hyper;
extern crate actix_web;
```

In top portion of the file (where the globals would be located after the __extern crates__ and __use__ sections), add the following global variable.

```
static VER: &str = "v1";
```

At the bottom of the file, add the following module as part of the library.

```
pub mod hello_world;
```

>**NOTE**: If you build the code right now, it would raise an error that the module cannot be found.
>
```
[user@localhost rust-daas]$ cargo run
   Compiling rust-daas v0.1.0 (C:\tmp\rust-daas)
error[E0583]: file not found for module `hello_world`
 --> src/lib.rs:5:9
  |
5 | pub mod hello_world;
  |         ^^^^^^^^^^^
  |
  = help: name the file either hello_world.rs or hello_world\mod.rs inside the directory "src"

error: aborting due to previous error

For more information about this error, try `rustc --explain E0583`.
error: Could not compile `rust-daas`.

To learn more, run the command again with --verbose.
```