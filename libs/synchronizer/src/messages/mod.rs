use bhomz::log_err;
use crate::messages::bytify::Bytify;
use crate::messages::commands::Commands;
use crate::messages::request::Requests;
use crate::messages::response::Response;
use crate::messages::transfers::Transfer;
use crate::messages::version::{to_byte_version, verify_version, VersionResult, VERSION};

pub mod commands;
pub mod request;
pub mod response;
pub mod transfers;
pub mod bytify;
mod version;

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub enum Msg<I>
where
	I: Bytify,
{
	Request {
		id: I,
		data: Requests
	},

	Response {
		id: I,
		data: Response<I>
	},

	Transfer {
		id: I,
		data: Transfer
	}
}

impl Msg<u64>
where
	Self: Sized,
{
	pub fn serialize(&self) -> anyhow::Result<Vec<u8>> {
		let mut msg = Vec::new();
		let mut version = to_byte_version(VERSION)?;
		let mut bytes = match self {
			Msg::Request { id, data } => {
				let mut bytes = id.to_sized_bytes();
				bytes.append(&mut data.serialize()?);
				bytes
			}
			Msg::Response { id, data } => {
				let mut bytes = id.to_sized_bytes();
				bytes.append(&mut data.serialize()?);
				bytes
			}
			Msg::Transfer { id, data } => {
				let mut bytes = id.to_sized_bytes();
				bytes.append(&mut data.serialize()?);
				bytes
			}
		};
		let mut size = (bytes.len() as u64).to_be_bytes().to_vec();
		msg.append(&mut version);
		msg.push(self.get_id());
		msg.append(&mut size);
		msg.append(&mut bytes);
		Ok(msg)
	}

	pub fn deserialize(bytes: &[u8]) -> anyhow::Result<Self> {
		if bytes.len() < 25 {
			return log_err!("Headers too short");
		}
		let version_result = verify_version(&bytes)?;
		if let VersionResult::Incompatible(major, minor, patch) = version_result {
			return log_err!("Incompatible version for parser: '{}' (data version: '{}.{}.{}')", VERSION, major, minor, patch)
		}
		let size = u64::from_be_bytes(bytes[9..17].try_into()?) as usize;
		let bytes = &bytes[0..17+size];
		match &bytes[8] {
			0 => {
				let (id, next) = u64::try_from_sized_bytes(&bytes, 17)?;
				let (data, _) = version_result.read_from_bytes(&bytes, next)?;
				Ok(Self::Request { id, data, })
			},

			1 => {
				let (id, next) = u64::try_from_sized_bytes(&bytes, 17)?;
				let (data, _) = version_result.read_from_bytes(&bytes, next)?;
				Ok(Self::Response { id, data, })
			},

			2 => {
				let (id, next) = u64::try_from_sized_bytes(&bytes, 17)?;
				let (data, _) = version_result.read_from_bytes(&bytes, next)?;
				Ok(Self::Transfer { id, data, })
			},

			_ => log_err!("Unknown msg id")
		}
	}

	fn get_id(&self) -> u8 {
		match self {
			Msg::Request { .. } => 0,
			Msg::Response { .. } => 1,
			Msg::Transfer { .. } => 2
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::messages::version::get_expected_with_version;
	use super::*;

	#[test]
	fn serializes() {
		let response = Response::<u64>::Acknowledged {
			msg_id: 93
		};
		let input = Msg::Response::<u64> {
			id: 64,
			data: response.clone()
		};
		let result = input.serialize();
		let expected = get_expected_with_version(vec![
			// Msg Type Id
			input.get_id(),
			// Size
			0, 0, 0, 0, 0, 0, 0, 17,

			// Msg id
			0, 0, 0, 0, 0, 0, 0, 64,
			// Response Id
			response.get_id(),
			// Ack msg_id
			0, 0, 0, 0, 0, 0, 0, 93,
		]);
		assert!(result.is_ok(), "Could not serialize {input:?}");
		let actual = result.unwrap();
		assert_eq!(actual, expected);

		let msg = Msg::deserialize(actual.as_slice());
		assert!(msg.is_ok(), "Could not deserialize {input:?} err: {msg:?}");
		assert_eq!(msg.unwrap(), input);
	}
}
