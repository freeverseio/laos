// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use crate::VERSION;

/// Converts a version string in the format "major.minor.patch" to an integer.
/// Each part of the version (major, minor, patch) must be a number less than 100.
/// The resulting integer is in the format major*10000 + minor*100 + patch.
///
/// # Arguments
/// * `version` - A string slice representing the version.
///
/// # Returns
/// A `Result` containing either the integer representation of the version
/// or an `Error` if the input format is incorrect or any part of the version
/// is 100 or greater.
///
/// # Examples
/// ```
/// let version = "1.2.3";
/// let int_version = version_to_int(version);
/// assert_eq!(int_version, 10203);
/// ```
fn parse_semantic_version(version: &str) -> u32 {
	let parts: Vec<u32> = version.split('.').map(|part| part.parse::<u32>().unwrap()).collect();
	assert!(parts.len() == 3, "Invalid version format");

	let major = parts[0];
	let minor = parts[1];
	let patch = parts[2];

	assert!(major < 100 && minor < 100 && patch < 100, "Version number must be less than 100");

	major * 10000 + minor * 100 + patch
}

#[test]
#[should_panic(expected = "Version number must be less than 100")]
fn version_member_is_100_should_error() {
	parse_semantic_version("100.0.0");
}

#[test]
fn standard_version() {
	assert_eq!(parse_semantic_version("1.2.3"), 10203);
}

#[test]
fn version_with_major_zero() {
	assert_eq!(parse_semantic_version("0.9.0"), 900);
}

#[test]
#[should_panic(expected = "Invalid version format")]
fn version_with_more_than_three_parts() {
	parse_semantic_version("1.2.3.4");
}

#[test]
#[should_panic(
	expected = "called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }"
)]
fn version_with_non_numeric_characters() {
	parse_semantic_version("a.b.c");
}

#[test]
#[should_panic(
	expected = "called `Result::unwrap()` on an `Err` value: ParseIntError { kind: Empty }"
)]
fn empty_version() {
	parse_semantic_version("");
}

#[test]
fn version_001_should_be_1() {
	assert_eq!(parse_semantic_version("0.0.1"), 1);
}

#[test]
fn runtime_version_should_be_derived_from_package_version() {
	let package_version = env!("CARGO_PKG_VERSION");
	let derived_runtime_version = parse_semantic_version(package_version);

	assert_eq!(
		derived_runtime_version, VERSION.spec_version,
		"Derived runtime version does not match expected spec_version"
	);
}
