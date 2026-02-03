use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use bhomz::log_err;

pub trait ConstSizeBytify
where
	Self: Sized + Bytify,
{
	fn get_size() -> usize;
}

pub trait Bytify
where
	Self: Sized,
{
	fn to_sized_bytes(&self) -> Vec<u8>;
	fn try_from_sized_bytes(bytes: &[u8], index: usize) -> anyhow::Result<(Self, usize)>;
}

impl Bytify for u64 {
	fn to_sized_bytes(&self) -> Vec<u8> {
		self.to_be_bytes().to_vec()
	}

	fn try_from_sized_bytes(data: &[u8], index: usize) -> anyhow::Result<(Self, usize)> {
		let last_index = index + u64::get_size();
		if data.len() < last_index {
			return log_err!("u64 cannot be parsed with less than 8 bytes");
		}
		Ok((u64::from_be_bytes(data[index..last_index].try_into()?), last_index))
	}
}

impl ConstSizeBytify for u64 {
	fn get_size() -> usize {
		8
	}
}

impl Bytify for String {
	fn to_sized_bytes(&self) -> Vec<u8> {
		let mut data = (self.len() as u64).to_be_bytes().to_vec();
		let mut bytes = self.clone().into_bytes().to_vec();
		data.append(&mut bytes);
		data
	}

	fn try_from_sized_bytes(bytes: &[u8], index: usize) -> anyhow::Result<(Self, usize)> {
		let mut cursor = index;
		let name_size = u64::from_be_bytes(bytes[cursor..cursor+8].try_into()?) as usize;
		cursor += 8;
		let name = String::from_utf8(bytes[cursor..cursor+ name_size].to_vec())?;
		cursor += name_size;
		Ok((name, cursor))
	}
}

impl<V> Bytify for Vec<V>
where
	V: Bytify + Ord + Clone,
{
	fn to_sized_bytes(&self) -> Vec<u8> {
		let mut data = vec![];
		let mut ordored = self.clone();
		ordored.sort();
		let mut size = (ordored.len() as u64).to_be_bytes().to_vec();
		data.append(&mut size);
		for version in ordored {
			let mut version_sized = version.to_sized_bytes();
			data.append(&mut version_sized);
		}
		data
	}

	fn try_from_sized_bytes(bytes: &[u8], index: usize) -> anyhow::Result<(Self, usize)> {
		let mut cursor = index;
		let mut values = Vec::new();
		let mut value_left = u64::from_be_bytes(bytes[cursor..cursor+8].try_into()?);
		cursor += 8;
		while value_left > 0 {
			let (version, new_cursor) = V::try_from_sized_bytes(bytes, cursor)?;
			cursor = new_cursor;
			values.push(version);
			value_left -= 1;
		}
		Ok((values, cursor))
	}
}

pub trait TryBytify
where
	Self: Sized,
{
	fn try_to_sized_bytes(&self) -> anyhow::Result<Vec<u8>>;
	fn try_from_sized_bytes(bytes: &[u8], index: usize) -> anyhow::Result<(Self, usize)>;
}

impl<K, V> TryBytify for HashMap<K, V>
where
	K: Bytify + Ord + Hash + Display,
	V: Bytify,
	Self: Sized,
{
	fn try_to_sized_bytes(&self) -> anyhow::Result<Vec<u8>> {
		let mut data = vec![];
		let mut names = self.keys().collect::<Vec<_>>();
		names.sort();
		for name in names {
			let mut name_sized = name.to_sized_bytes();
			data.append(&mut name_sized);
			match self.get(name) {
				Some(values) => {
					let mut name_sized = values.to_sized_bytes();
					data.append(&mut name_sized);
				},
				None => return log_err!("Dataset '{}' does not have a version", name)
			};
		}
		Ok(data)
	}

	fn try_from_sized_bytes(bytes: &[u8], index: usize) -> anyhow::Result<(Self, usize)> {
		let mut cursor = index;
		let mut map = HashMap::new();
		let mut dataset_left = u64::from_be_bytes(bytes[cursor..cursor+8].try_into()?) as usize;
		cursor += 8;
		while dataset_left > 0 {
			let (key, new_cursor) = K::try_from_sized_bytes(bytes, cursor)?;
			cursor = new_cursor;

			let (value, new_cursor) = V::try_from_sized_bytes(bytes, cursor)?;
			cursor = new_cursor;

			map.insert(key, value);
			dataset_left -= 1;
		}
		Ok((map, cursor))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn u64_conversion() {
		let num: u64 = u64::MAX;
		let bytes = num.to_sized_bytes();
		let (actual, next) = u64::try_from_sized_bytes(&bytes, 0).unwrap();
		assert_eq!(num, actual);
		assert_eq!(next, 8);
	}
}