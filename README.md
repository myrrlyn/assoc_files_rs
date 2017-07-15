# `assoc_files`

Including files directly within one's compilation artifact is so passé.

With this library in your crate's build dependencies, your build script can copy
files you need at runtime into predefined or custom destinations, and with this
library in your crate's normal dependencies, you can rapidly retrieve those same
files at runtime.

## Build Script Usage

Set up your `Cargo.toml` to have a build script and pull in this crate:

```toml
[package]
build = "build.rs"

[build-dependencies.assoc_files]
version = "0.1"
git = "https://github.com/myrrlyn/assoc_files_rs.git"
```

Make some files you'll use at runtime, and would normally `include_str!()` into
your crate:

```text
crate_root/
  data/
    sample.tt
    templates/
      foo.tt
      bar.tt
      ...
    settings.yml
  src/
    main.rs
  build.rs
```

In your build script (by convention, `build.rs`):

```rust
#[macro_use]
extern crate assoc_files;

use assoc_files;

fn main() {
    user_data![
        "data/sample.tt",
        "data/templates",
    ].unwrap();
    user_config![
        "data/settings.yml",
    ].unwrap();
}
```

At compile time, the contents of `data/` will be copied into your local user's
configuration and data directories, according to operating system convention.

## Runtime Usage

Now, to access these files, you add this crate as a normal dependency to
`Cargo.toml`

```toml
[dependencies.assoc_files]
version = "0.1"
git = "https://github.com/myrrlyn/assoc_files_rs.git"
```

and in your crate's runtime logic, the same macros used in the build script will
provide the locations where those files were installed:

```rust
fn load_config() {
    let cfgdir = user_config!();
    let mut config = String::new();
    let mut cfg = File::open(cfgdir.join("settings.yml")).unwrap();
    cfg.read_to_string(&mut config).unwrap();
}

fn load_sample_template() {
    let datadir = user_data!();
    let mut sample = String::new();
    let mut smp = File::open(datadir.join("sample.tt")).unwrap();
    smp.read_to_string(&mut sample).unwrap();
}
```

If you don't want to use the predetermined directories (which are the local
convention, followed by your crate name and then version), you can also use the
`install_files()` function directly: It takes a list of paths: `&[AsRef<Path>]`
and an installation target: `&Path`, and copies the given files and directory
contents into the target. This target directory can then be read at runtime
using the tools in `std::fs` to retrieve the installed files, as shown above.

## Caveats

This crate does not provide equivalent macros for the system directories, as
compilation is typically not done with a user that has write permissions there.
If you plan on distributing your crate in such a way that it will draw on system
locations, then it is up to you to make sure those files are available. This
concern is unrelated to the problem this crate aims to solve, and so I have made
no attempt to solve it here.
