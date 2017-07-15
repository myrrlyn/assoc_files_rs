/*! Associated Files

This library is designed to be used within the build script of a crate, where it
will scan files or folders within your source tree and install them into system
appropriate locations *at compile time*. This is akin to installing associated
files during system package installation, but for `cargo install` or other
build procedures.

The four exposed functions take sets of paths within the crate source and copy
those paths into the appropriate system location, as determined by the client
project name and the choice of function.

The functions can accept paths to files or directiories. Paths to files will
place the file in the top level of the target. Paths to directories will keep
interior structure of the directory, but not copy over the root. That is, for a
directory layout:

```text
data/
  foo.txt
  bar/
    baz.txt
base/
  readme.md.tt
```

If an install function is called with `&["base/readme.md.tt", "data/"]`, then
after installation the target folder will contain the following structure:

```text
foo.txt
bar/
  baz.txt
readme.md.tt
```

The contents of `data/`, and the `readme.md.tt` file, have been copied into the
target. Their original paths within the crate source are not maintained.
!*/

#![macro_export]

extern crate appdirs;
extern crate walkdir;

use std::fs::{
	self,
};
use std::io::{
	self,
};
use std::path::{
	Path,
};
use walkdir::{
	WalkDir,
};

/// Installs directly to `$user_config/$crate/$version`.
///
/// This macro takes a list of items, much like `vec![]`. The items given must
/// be usable as paths (strictly speaking, they must all satisfy `AsRef<Path>`),
/// and will almost always be `&str` literals.
///
/// If it is called without parameters, it simply returns the installation
/// directory.
///
/// # Examples
///
/// ## Installation at Compile Time
///
/// For a source directory:
///
/// ```text
/// sample/
///   foo.txt
///   data/
///     bar.txt
///     baz/
///       quux.txt
/// ```
///
/// then the build script should call
///
/// ```rust,ignore
/// user_config!["sample/foo.txt", "sample/data"];
/// ```
///
/// to copy `sample/foo.txt` and the contents below `sample/data/` into the
/// current user's configuration directory. With this call, `foo.txt`,
/// `bar.txt`, and `baz/` will be siblings in the installed location.
///
/// ## Accessing at Run Time
///
/// In your crate's logic, the installed files can be accessed like so:
///
/// ```rust,ignore
/// let config_dir: PathBuf = user_config!();
/// let foo = File::open(config_dir.join("foo.txt")).unwrap();
/// let quux = File::open(config_dir.join("baz").join("quux.txt")).unwrap();
/// ```
#[macro_export]
macro_rules! user_config {
	($($f:expr),+) => {{
		let arr = [
			$($f),+
		];
		let cn = env!("CARGO_PKG_NAME");
		let cv = env!("CARGO_PKG_VERSION");
		let base = appdirs::user_config_dir(Some(cn), None, false).unwrap();
		let ver = base.join(cv);
		install_files(&arr, &ver)
	}};

	() => {
		appdirs::user_config_dir(
			Some(env!("CARGO_PKG_NAME")),
			None,
			false,
		).unwrap().join(env!("CARGO_PKG_VERSION"))
	};
}

/// Installs directly to `$user_data/$crate/$version`.
///
/// This macro takes a list of items, much like `vec![]`. The items given must
/// be usable as paths (strictly speaking, they must all satisfy `AsRef<Path>`),
/// and will almost always be `&str` literals.
///
/// If it is called without parameters, it simply returns the installation
/// directory.
///
/// # Examples
///
/// ## Installation at Compile Time
///
/// For a source directory:
///
/// ```text
/// sample/
///   foo.txt
///   data/
///     bar.txt
///     baz/
///       quux.txt
/// ```
///
/// then the build script should call
///
/// ```rust,ignore
/// user_data!["sample/foo.txt", "sample/data"];
/// ```
///
/// to copy `sample/foo.txt` and the contents below `sample/data/` into the
/// current user's data directory. With this call, `foo.txt`, `bar.txt`, and
/// `baz/` will be siblings in the installed location.
///
/// ## Accessing at Run Time
///
/// In your crate's logic, the installed files can be accessed like so:
///
/// ```rust,ignore
/// let data_dir: PathBuf = user_data!();
/// let foo = File::open(data_dir.join("foo.txt")).unwrap();
/// let quux = File::open(data_dir.join("baz").join("quux.txt")).unwrap();
/// ```
#[macro_export]
macro_rules! user_data {
	($($f:expr),+) => {{
		let paths = [
			$($f),+
		];
		let cn = env!("CARGO_PKG_NAME");
		let cv = env!("CARGO_PKG_VERSION");
		let base = appdirs::user_data_dir(Some(cn), None, false).unwrap();
		let ver = base.join(cv);
		install_files(&paths, &ver)
	}};

	() => {
		appdirs::user_data_dir(
			Some(env!("CARGO_PKG_NAME")),
			None,
			false,
		).unwrap().join(env!("CARGO_PKG_VERSION"))
	};
}

/// Installs the given files into a destination within the system.
///
/// This function assumes that all files named in the paths array should be
/// placed in the top level of the target, while all directories have their
/// contents copied while maintaining interior structure.
///
/// A successful return carries the count of installed files.
///
/// # Examples
///
/// ## Installation at Compile Time
///
/// For a source directory:
///
/// ```text
/// sample/
///   foo.txt
///   data/
///     bar.txt
///     baz/
///       quux.txt
/// ```
///
/// and a call:
///
/// ```rust,ignore
/// let dir = Path::new("/tmp/foo");
/// install_files(&["sample/foo.txt", "sample/data"], &dir);
/// ```
///
/// then the path `/tmp/foo` will be populated with:
///
/// ```text
/// foo.txt
/// bar.txt
/// baz/
///   quux.txt
/// ```
///
/// ## Accessing at Run Time
///
/// In your crate's logic, the installed files can be accessed at the directory
/// given in your build script using `std::fs`.
pub fn install_files<P>(paths: &[P], dest: &Path) -> io::Result<usize>
	where P: AsRef<Path> {
	if !dest.exists() {
		fs::create_dir_all(&dest)?;
	}
	let mut count = 0;
	for p in paths {
		let p = p.as_ref();
		//  For any directories, their *contents* are copied as-is into the
		//  target location. This means stripping the given path prefix out of
		//  the contents of the directory (seen below).
		if p.is_dir() {
			//  Recurse through the directory.
			for entry in WalkDir::new(p).into_iter().filter_map(|e| e.ok()) {
				/// Path within the source location, p/entry
				let full_path = entry.path();
				/// Relative path inside the source root dir, with p stripped.
				let inner_path = full_path.strip_prefix(p).unwrap_or(full_path);
				/// Final target: destination/entry.
				let dest_path = dest.join(inner_path);
				//  If the entry's full path (in the source location) is a
				//  directory, make a corresponding directory at target/inner
				if full_path.is_dir() {
					fs::create_dir_all(dest_path)?;
				}
					//  Otherwise, if the full path in source is a file, copy it
					//  into the target location.
					else if full_path.is_file() {
						fs::copy(full_path, dest_path)?;
						count += 1;
					}
			}
		}
			//  For any files, the file gets copied directly into the target root.
			else if p.is_file() {
				let inner_path = p.file_name().unwrap();
				let dest_path = dest.join(inner_path);
				fs::copy(p, dest_path)?;
				count += 1;
			}
	}
	Ok(count)
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::path::{
		Path,
	};
	#[test]
	fn install_custom() {
		let target = Path::new("target/tmp");
		if target.exists() {
			if target.is_dir() {
				fs::remove_dir_all(target).unwrap();
			}
				else if target.is_file() {
					fs::remove_file(target).unwrap();
				}
		}
		fs::create_dir_all(target).unwrap();

		let res = install_files(&["data/foo.txt", "data/data"], &target.to_path_buf());
		assert!(res.is_ok());
		assert_eq!(res.ok(), Some(3));

		fs::remove_dir_all(target).unwrap();
	}
}
