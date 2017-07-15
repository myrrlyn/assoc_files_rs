/*! Test installing to user_{config,data}_dir() locations
!*/

extern crate appdirs;
#[macro_use]
extern crate assoc_files;

use appdirs::*;
use assoc_files::*;
use std::fs::remove_dir_all;

#[test]
fn user_config_macro() {
	let ret = user_config![
		"data/foo.txt",
		"data/data/"
	];
	assert!(ret.is_ok());
	assert_eq!(ret.ok(), Some(3));
	let target_dir = user_config_dir(Some(env!("CARGO_PKG_NAME")), None, false).unwrap().join(env!("CARGO_PKG_VERSION"));
	for file in &["foo.txt", "bar.txt", "baz/quux.txt"] {
		assert!(target_dir.join(file).exists());
	}
	remove_dir_all(target_dir).unwrap();
}

#[test]
fn user_data_macro() {
	let ret = user_data![
		"data/foo.txt",
		"data/data/"
	];
	assert!(ret.is_ok());
	assert_eq!(ret.ok(), Some(3));
	let target_dir = user_data_dir(Some(env!("CARGO_PKG_NAME")), None, false).unwrap().join(env!("CARGO_PKG_VERSION"));
	for file in &["foo.txt", "bar.txt", "baz/quux.txt"] {
		assert!(target_dir.join(file).exists());
	}
	remove_dir_all(target_dir).unwrap();
}
