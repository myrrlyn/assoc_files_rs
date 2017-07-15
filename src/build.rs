/*! Compile-time file installation

The `install_files()` function (and the `user_config!` and `user_data!` macros
which wrap it) copy files from the source tree of the client crate and into a
system location from which they will be accessible at runtime.

This is accomplished by adding this crate as a build dependency in `Cargo.toml`:

```toml
[package]
build = "build.rs"

[build-dependencies]
assoc_files = "1"
```

and using its exports within your build script:

```rust,ignore
//  build.rs

#[macro_use]
extern crate assoc_files;

use assoc_files::build::*;

fn main() {
    //  Install all your templates to `$crate/$ver`
    user_data![
        "templates",
    ].unwrap();
    //  Install your default config file to `$crate/$ver`
    user_config![
        "data/config.toml",
    ].unwrap();
    //  Install another configuration in `~/.cargo`
    install_files(
        &["data/mycrate.toml"],
        &::std::env::home_dir().unwrap().join(".cargo")
    ).unwrap();
}
```

In your crate's main logic, it can now access these files by using macros
provided in the `run` module.
!*/
