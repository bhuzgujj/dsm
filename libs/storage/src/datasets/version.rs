use anyhow::Error;
use rusqlite::{params, Connection};
use interfaces::models::Datasets;
use crate::datasets::{classes, data_entries};

const QUERY_VERSION_BY_VERSION: &'static str = "SELECT id FROM versions WHERE versions = ?1 AND types = ?2 AND datasets_id = ?3;";
const INSERT_NEW_VERSION: &'static str = "INSERT INTO versions(versions, types, datasets_id) VALUES(?1, ?2, ?3) RETURNING ROWID;";

pub(crate) fn store(connection: &Connection, dataset_id: i32, datasets: &Datasets) -> anyhow::Result<()> {
	let mut stmt = connection.prepare(QUERY_VERSION_BY_VERSION)?;

	for (name, entries) in datasets.get_entries() {
		let mut result = stmt.query(params![&datasets.get_version(), &name, dataset_id])?;
		let mut count = 0;
		let mut id = 0;
		while let Some(row) = result.next()? {
			id = row.get(0)?;
			count += 1;
		}
		if count == 0 {
			let mut stmt = connection.prepare(INSERT_NEW_VERSION)?;
			id = stmt.query_row(params![&datasets.get_version(), &name, dataset_id], |r| r.get(0))?;
			stmt.finalize()?;
			let class_mapping = classes::store(connection, id, datasets.get_classes())?;
			for entry in entries {
				data_entries::store(&connection, id, entry, &class_mapping)?;
			}
		} else {
			return Err(Error::msg(format!("Version already exists with id: {}", id)))
		}
	}
	Ok(())
}