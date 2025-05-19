use std::collections::HashMap;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use interfaces::models::classes::DsmClasses;

pub(crate) const FILE_NAME: &str = "obj.names";

pub(crate) fn read(path: PathBuf) -> anyhow::Result<HashMap<u32, DsmClasses>> {
	let names = read_to_string(path)?;
	let mut classes = HashMap::new();
	let mut index: u32 = 0;
	for line in names.lines() {
		let trimmed = line.trim();
		if !trimmed.is_empty() {
			classes.insert(index, DsmClasses::new(trimmed.to_string(), None));
			index += 1;
		}
	}
	Ok(classes)
}

pub(crate) fn write(root: &PathBuf, classes: HashMap<u32, DsmClasses>) -> anyhow::Result<()> {
	let vec: Vec<String> = classes.values()
		.map(|s| s.get_classes().clone())
		.collect();
	OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(root.join(FILE_NAME))?
		.write_all(vec.join("\n").as_bytes())?;
	Ok(())
}