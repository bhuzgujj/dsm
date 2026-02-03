use std::collections::HashMap;
use bhomz::log_err;
use crate::messages::commands::Commands;
use crate::messages::bytify::{Bytify, TryBytify};

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
pub enum Requests {
	Sync {
		current: HashMap<String, Vec<String>>
	},
}

impl Requests {
	pub(crate) fn get_id(&self) -> u8 {
		match self {
			Requests::Sync { .. } => 0,
		}
	}
}

impl Commands for Requests {
	fn serialize(&self) -> anyhow::Result<Vec<u8>> {
		let mut data = vec![self.get_id()];
		match self {
			Requests::Sync { current } => {
				let mut size = (current.len() as u64).to_be_bytes().to_vec();
				data.append(&mut size);
				for (key, val) in current {
					if val.is_empty() {
						return log_err!("Dataset '{}' need at least one version", key)
					}
				}
				let mut sized = current.try_to_sized_bytes()?;
				data.append(&mut sized);
			}
		}
		Ok(data)
	}

	fn deserialize(bytes: &[u8], index: usize) -> anyhow::Result<(Self, usize)> {
		match &bytes[index] {
			0 => {
				let (current, cursor) = HashMap::try_from_sized_bytes(bytes, index + 1)?;
				Ok((Requests::Sync { current }, cursor))
			},

			id => log_err!("Unknown request id: '{}'", id),
		}
	}
}

#[cfg(test)]
mod tests {
	mod sync {
		use std::collections::HashMap;
		use crate::assert_unordered_eq;
		use crate::messages::commands::Commands;
		use crate::messages::request::Requests;

		fn empty() -> (Requests, Vec<u8>) {
			let request = Requests::Sync { current: HashMap::new() };
			let id = request.get_id();
			(
				request,
				vec![
					id,
					// Length
					0, 0, 0, 0, 0, 0, 0, 0,
				]
			)
		}

		fn single() -> (Requests, Vec<u8>) {
			let request = Requests::Sync {
				current: HashMap::from([
					("lmao".to_string(), vec!["1".to_string()]),
				])
			};
			let id = request.get_id();
			(
				request,
				vec![
					id,
					// Dataset Count
					0, 0, 0, 0, 0, 0, 0, 1,

					// Name len
					0, 0, 0, 0, 0, 0, 0, 4,
					// Name
					108, 109, 97, 111,

					// Versions Count
					0, 0, 0, 0, 0, 0, 0, 1,
					// Version len
					0, 0, 0, 0, 0, 0, 0, 1,
					// Version
					49,
				]
			)
		}

		fn multi_version() -> (Requests, Vec<u8>) {
			let request = Requests::Sync {
				current: HashMap::from([
					("xd".to_string(), vec!["3".to_string(), "2".to_string()]),
				])
			};
			let id = request.get_id();
			(
				request,
				vec![
					id,
					// Dataset Count
					0, 0, 0, 0, 0, 0, 0, 1,

					// Name len
					0, 0, 0, 0, 0, 0, 0, 2,
					// Name
					120, 100,

					// Versions Count
					0, 0, 0, 0, 0, 0, 0, 2,
					// Version len
					0, 0, 0, 0, 0, 0, 0, 1,
					// Version
					50,
					// Version len
					0, 0, 0, 0, 0, 0, 0, 1,
					// Version
					51,
				]
			)
		}

		fn multi() -> (Requests, Vec<u8>) {
			let request = Requests::Sync {
				current: HashMap::from([
					("fef".to_string(), vec!["f".to_string()]),
					("xd".to_string(), vec!["3".to_string(), "2".to_string()]),
				])
			};
			let id = request.get_id();
			(
				request,
				vec![
					id,
					// Dataset Count
					0, 0, 0, 0, 0, 0, 0, 2,

					// Name len
					0, 0, 0, 0, 0, 0, 0, 3,
					// Name
					102, 101, 102,

					// Versions Count
					0, 0, 0, 0, 0, 0, 0, 1,
					// Version len
					0, 0, 0, 0, 0, 0, 0, 1,
					// Version
					102,

					// Name len
					0, 0, 0, 0, 0, 0, 0, 2,
					// Name
					120, 100,

					// Versions Count
					0, 0, 0, 0, 0, 0, 0, 2,
					// Version len
					0, 0, 0, 0, 0, 0, 0, 1,
					// Version
					50,
					// Version len
					0, 0, 0, 0, 0, 0, 0, 1,
					// Version
					51,
				]
			)
		}

		#[test]
		fn empty_works() {
			let (request, expected) = empty();
			let data = assert_serde_request(&request, expected);
			assert_deserde_request(&data, &request);
		}

		#[test]
		fn with_a_dataset_info_without_version() {
			let request = Requests::Sync {
				current: HashMap::from([
					("lmao".to_string(), Vec::new())
				])
			};
			assert!(request.serialize().is_err());
		}

		#[test]
		fn with_a_dataset_info() {
			let (request, expected) = single();
			let data = assert_serde_request(&request, expected);
			assert_deserde_request(&data, &request);
		}

		#[test]
		fn with_a_dataset_info_multi_version() {
			let (request, expected) = multi_version();
			let data = assert_serde_request(&request, expected);
			assert_deserde_request(&data, &request);
		}

		#[test]
		fn with_a_dataset_info_multi_version_and_names() {
			let (request, expected) = multi();
			let data = assert_serde_request(&request, expected);
			assert_deserde_request(&data, &request);
		}

		fn assert_serde_request(request: &Requests, expected: Vec<u8>) -> Vec<u8> {
			let result = request.serialize();
			assert!(result.is_ok(), "Could not serialize request: {request:?}");
			let data = result.unwrap();
			assert_eq!(data, expected);
			data
		}

		fn assert_deserde_request(actual: &Vec<u8>, expected: &Requests) {
			let deserd = Requests::deserialize(actual.as_slice(), 0);
			assert!(deserd.is_ok(), "Could not deserialize request: {deserd:?}");
			let (req, _) = deserd.unwrap();
			assert_eq!(req.get_id(), 0);
			let Requests::Sync { current: actual } = req;
			let Requests::Sync { current: expected } = expected;
			assert_eq!(actual.len(), expected.len());
			for (k, v) in expected {
				let values = actual.get(k);
				assert!(values.is_some(), "Does not have dataset named: {k}");
				assert_unordered_eq(values.unwrap(), &v);
			}
		}
	}
}