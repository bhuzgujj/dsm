use bhomz::log_err;
use log::warn;
use crate::messages::commands::Commands;

pub(crate) const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
pub(crate) fn get_expected_with_version(msg_bytes: Vec<u8>) -> Vec<u8> {
	let mut expected = to_byte_version(VERSION).unwrap();
	expected.append(&mut msg_bytes.clone());
	expected
}

#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub(crate) enum VersionResult {
	Same,
	Compatible(u16, u16, u32),
	Incompatible(u16, u16, u32),
}

impl VersionResult {
	pub(crate) fn read_from_bytes<T: Commands + Sized>(&self, bytes: &[u8], index: usize) -> anyhow::Result<(T, usize)> {
		match self {
			VersionResult::Same => T::deserialize(&bytes, index),
			VersionResult::Compatible(major, minor, patch) => {
				match T::deserialize(&bytes, index) {
					Ok(val) => Ok(val),
					Err(err) => {
						warn!("Error can be caused by a version missmatch (parser: '{}', msg version: '{}.{}.{}'", VERSION, major, minor , patch);
						Err(err)
					}
				}
			}
			VersionResult::Incompatible(major, minor, patch) => log_err!("Parser version '{}' cannot parse '{}.{}.{}'", VERSION, major, minor , patch),
		}
	}
}

pub(crate) fn to_byte_version(version_str: &str) -> anyhow::Result<Vec<u8>> {
	let (major, minor, patch) = from_version_str(version_str)?;
	Ok(((major << 48) | (minor << 32) | patch).to_be_bytes().to_vec())
}

pub(crate) fn from_version_str(version_str: &str) -> anyhow::Result<(u64, u64, u64)> {
	let version = version_str.split(".").collect::<Vec<&str>>();
	if version.len() != 3 {
		return log_err!("Invalid version: {}", VERSION);
	}
	let major = version[0].parse::<u64>()?;
	let minor = version[1].parse::<u64>()?;
	let patch = version[2].parse::<u64>()?;
	Ok((major, minor, patch))
}

pub(crate) fn from_version_bytes(version: &[u8]) -> anyhow::Result<(u16, u16, u32)> {
	if version.len() < 8 {
		return log_err!("Invalid version: {}", VERSION);
	}
	let major = u16::from_be_bytes([version[0], version[1]]);
	let minor = u16::from_be_bytes([version[2], version[3]]);
	let patch = u32::from_be_bytes([version[4], version[5], version[6], version[7]]);
	Ok((major, minor, patch))
}

pub(crate) fn verify_version(version: &[u8]) -> anyhow::Result<VersionResult> {
	let (major, minor, patch) = from_version_bytes(version)?;
	let (curr_major, curr_minor, curr_patch) = from_version_str(VERSION)?;
	if curr_major > major as u64 {
		return Ok(VersionResult::Incompatible(major, minor, patch))
	}
	if curr_minor > minor as u64 {
		return Ok(VersionResult::Compatible(major, minor, patch))
	}
	if curr_patch > patch as u64 {
		return Ok(VersionResult::Compatible(major, minor, patch))
	}
	Ok(VersionResult::Same)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn convert_to_byte_version() {
		assert!(to_byte_version("1").is_err());
		assert!(to_byte_version("1.").is_err());
		assert!(to_byte_version("1.3").is_err());
		assert!(to_byte_version("1.3.").is_err());
		for (input, output) in [
			("1.2.3", vec![0, 1, 0, 2, 0, 0, 0, 3]),
			("10.2.3", vec![0, 10, 0, 2, 0, 0, 0, 3]),
			("0.1.0", vec![0, 0, 0, 1, 0, 0, 0, 0]),
		] {
			let res = to_byte_version(input);
			assert!(res.is_ok(), "{input} cannot be parsed");
			assert_eq!(res.unwrap(), output);
		}
	}

	#[test]
	fn convert_current_version_to_and_from_bytes() {
		let bytes = to_byte_version(VERSION).unwrap();
		let version = verify_version(&bytes[0..8]).unwrap();
		assert_eq!(version, VersionResult::Same);
	}
}