mod version;
mod data_entries;
mod classes;

use std::collections::HashMap;
use std::path::PathBuf;
use rusqlite::{params, Connection};
use interfaces::models::Datasets;
use interfaces::models::metadata::MetaData;

pub(crate) fn store(path: &PathBuf, datasets: &Datasets) -> anyhow::Result<()> {
	let connection = Connection::open(path)?;
	connection.execute("BEGIN TRANSACTION;", [])?;

	match insert_database(&datasets, &connection) {
		Ok(()) => {
			//connection.execute("ROLLBACK TRANSACTION;", [])?;
			connection.execute("COMMIT TRANSACTION;", [])?;
			Ok(())
		},
		Err(e) => {
			connection.execute("ROLLBACK TRANSACTION;", [])?;
			Err(e)
		}
	}
}

fn insert_database(datasets: &&Datasets, connection: &Connection) -> anyhow::Result<()> {
	let mut stmt = connection.prepare("SELECT id FROM datasets WHERE names = ?1;")?;
	let mut result = stmt.query(&[&datasets.get_name()])?;
	let mut count = 0;
	let mut id = 0;
	while let Some(row) = result.next()? {
		id = row.get(0)?;
		count += 1;
	}

	if count == 0 {
		let mut stmt = connection.prepare("INSERT INTO datasets(names) VALUES(?1) RETURNING ROWID;")?;
		id = stmt.query_row(params![&datasets.get_name()], |r| r.get(0))?;
		stmt.finalize()?;
	}

	version::store(&connection, id, datasets)?;
	Ok(())
}

struct Version {
	id: u32,
	version: u32,
}

pub(crate) fn read(path: &PathBuf, name: String, version: u32) -> anyhow::Result<Datasets> {
	let connection = Connection::open(path)?;
	let mut stmt = connection.prepare("SELECT id, names FROM datasets WHERE names = ?1;")?;
	let (dataset_id, dataset_name): (u32, String) = stmt.query_row(params![&name], |r| {
		return Ok((r.get(0)?, r.get(1)?))
	})?;
	stmt.finalize()?;
	let (class, mapping) = classes::read(&connection, version)?;

	let mut stmt = connection.prepare("SELECT id, types FROM versions WHERE datasets_id = ?1 AND versions = ?2;")?;
	let versions = stmt.query_map(params![&dataset_id, version], |r| {
		let id: i32 = r.get(0)?;
		let types: String = r.get(1)?;
		return Ok((id, types))
	})?;
	let mut sets = HashMap::new();
	for version in versions {
		let (id, types) = version?;
		let entries = data_entries::read(&connection, id, &mapping)?;
		sets.insert(types, entries);
	}
	let metadata = MetaData::new(dataset_name, version, class);
	Ok(Datasets::new(metadata, sets))
}