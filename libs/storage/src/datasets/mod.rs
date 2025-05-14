mod version;
mod data_entries;

use rusqlite::{params, Connection};
use interfaces::models::Datasets;

const QUERY_DATASETS: &'static str = "SELECT * FROM datasets;";
const QUERY_DATASETS_ID_FROM_NAME: &'static str = "SELECT id FROM datasets WHERE names = ?1;";
const INSERT_NEW_DATASETS: &'static str = "INSERT INTO datasets(names) VALUES(?1) RETURNING ROWID;";

pub(crate) fn store(path: &std::path::PathBuf, datasets: &Datasets) -> anyhow::Result<()> {
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
	let mut stmt = connection.prepare(QUERY_DATASETS_ID_FROM_NAME)?;
	let mut result = stmt.query(&[&datasets.get_name()])?;
	let mut count = 0;
	let mut id = 0;
	while let Some(row) = result.next()? {
		id = row.get(0)?;
		count += 1;
	}

	if count == 0 {
		let mut stmt = connection.prepare(INSERT_NEW_DATASETS)?;
		id = stmt.query_row(params![&datasets.get_name()], |r| r.get(0))?;
		stmt.finalize()?;
	}

	version::store(&connection, id, datasets)?;
	Ok(())
}