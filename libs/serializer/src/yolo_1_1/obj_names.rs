use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

pub(crate) fn read(path: PathBuf) -> anyhow::Result<HashMap<u32, String>> {
	let names = read_to_string(path)?;
	let mut classes = HashMap::new();
	let mut index: u32 = 0;
	for line in names.lines() {
		let trimmed = line.trim();
		if trimmed.len() > 0 {
			classes.insert(index, trimmed.to_string());
			index += 1;
		}
	}
	Ok(classes)
}