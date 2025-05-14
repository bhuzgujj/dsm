use rusqlite::{params, Connection};
use std::collections::HashMap;

pub(crate) fn store(connection: &Connection, version_id: i32, classes: &HashMap<u32, String>) -> anyhow::Result<HashMap<u32, i32>> {
	let mut mapping: HashMap<u32, i32> = HashMap::new();
	for (id, class) in classes {
		let mut stmt = connection.prepare("INSERT INTO classes(names, version_id) VALUES(?1, ?2) RETURNING ROWID;")?;
		let new_id = stmt.query_row(params![&class, version_id], |r| r.get(0))?;
		mapping.insert(*id, new_id);
		stmt.finalize()?;
	}
	Ok(mapping)
}

pub(crate) fn read(connection: &Connection, version_id: u32) -> anyhow::Result<(HashMap<u32, String>, HashMap<i32, u32>)> {
	let mut stmt = connection.prepare("SELECT id, names FROM classes WHERE version_id = ?1;")?;
	let versions = stmt.query_map(params![&version_id], |r| {
		let id: i32 = r.get(0)?;
		let names: String = r.get(1)?;
		return Ok((id, names))
	})?;
	let mut classes: HashMap<u32, String> = HashMap::new();
	let mut mapping: HashMap<i32, u32> = HashMap::new();
	let mut i = 0;
	for version in versions {
		let (id, names) = version?;
		let _ = classes.insert(i, names);
		let _ = mapping.insert(id, i);
		i += 1;
	}
	Ok((classes, mapping))
}