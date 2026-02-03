use crate::messages::bytify::Bytify;
use crate::messages::commands::Commands;

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
pub enum Transfer {
	File {
		info: Info,
		filename: String,
		relpath: String,
		data: Vec<u8>
	},
}

impl Commands for Transfer {
	fn serialize(&self) -> anyhow::Result<Vec<u8>> {
		let mut bytes = Vec::new();
		match self {
			Transfer::File {
				info,
				filename,
				relpath,
				data
			} => {
				bytes.append(&mut info.to_sized_bytes());
				bytes.append(&mut filename.to_sized_bytes());
				bytes.append(&mut relpath.to_sized_bytes());
				bytes.append(&mut data.to_vec());
			}
		}
		Ok(bytes)
	}

	fn deserialize(bytes: &[u8], index: usize) -> anyhow::Result<(Self, usize)> {
		todo!()
	}
}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
pub struct Info {
	name: String,
	version: String,
	types: InfoType,
}

impl Bytify for Info {
	fn to_sized_bytes(&self) -> Vec<u8> {
		let mut bytes = vec![self.types.get_id()];
		bytes.append(&mut self.name.to_sized_bytes());
		bytes.append(&mut self.version.to_sized_bytes());
		bytes
	}

	fn try_from_sized_bytes(data: &[u8], index: usize) -> anyhow::Result<(Self, usize)> {
		todo!()
	}
}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
pub enum InfoType {
	Dataset,
	MergedSet
}

impl InfoType {
	fn get_id(&self) -> u8 {
		match self {
			InfoType::Dataset => 0,
			InfoType::MergedSet => 1,
		}
	}
}
