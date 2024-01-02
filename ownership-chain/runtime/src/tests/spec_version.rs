use crate::VERSION;
use std::{
	io::{Error, ErrorKind},
	num::ParseIntError,
};

fn version_to_int(version: &str) -> Result<u32, Error> {
	let parts: Vec<&str> = version.split('.').collect();
	if parts.len() != 3 {
		return Err(Error::new(ErrorKind::InvalidInput, "Invalid version format"));
	}

	let major = parts[0]
		.parse::<u32>()
		.map_err(|e: ParseIntError| Error::new(ErrorKind::InvalidInput, e))?;
	let minor = parts[1]
		.parse::<u32>()
		.map_err(|e: ParseIntError| Error::new(ErrorKind::InvalidInput, e))?;
	let patch = parts[2]
		.parse::<u32>()
		.map_err(|e: ParseIntError| Error::new(ErrorKind::InvalidInput, e))?;

	Ok(major * 10000 + minor * 100 + patch)
}

#[test]
fn test_standard_version() {
	assert_eq!(version_to_int("1.2.3").unwrap(), 10203);
}

#[test]
fn test_version_with_leading_zeros() {
	assert_eq!(version_to_int("0.9.0").unwrap(), 900);
}

#[test]
fn test_version_with_more_than_three_parts() {
	match version_to_int("1.2.3.4") {
		Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidInput),
		_ => panic!("Expected an InvalidInput error"),
	}
}

#[test]
fn test_version_with_non_numeric_characters() {
	match version_to_int("a.b.c") {
		Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidInput),
		_ => panic!("Expected an InvalidInput error"),
	}
}

#[test]
fn test_empty_version() {
	match version_to_int("") {
		Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidInput),
		_ => panic!("Expected an InvalidInput error"),
	}
}

#[test]
fn test_0_0_1_should_be_1() {
	assert_eq!(version_to_int("0.0.1").unwrap(), 1);
}

#[test]
fn test_runtime_version_should_be_derived_from_package_version() {
	let package_version = env!("CARGO_PKG_VERSION");
	let derived_runtime_version = version_to_int(package_version);

	assert!(derived_runtime_version.is_ok(), "Version conversion failed");

	assert_eq!(
		derived_runtime_version.unwrap(),
		VERSION.spec_version,
		"Derived runtime version does not match expected spec_version"
	);
}
