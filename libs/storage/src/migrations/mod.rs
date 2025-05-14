mod migrations;

use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Error;
use rusqlite::{params, Connection};
use log::{info, trace};
use crate::migrations::migrations::Migration;

struct References {
	id: u64,
	up: &'static str,
	down: &'static str,
}

const INITIALIZE: &'static str = include_str!("../../migrations/init.sql");
const REFERENCES: &[References] = &[
	References {
		id: 1,
		up: include_str!("../../migrations/up/add_datasets.sql"),
		down: include_str!("../../migrations/down/add_datasets.sql"),
	},
	References {
		id: 2,
		up: include_str!("../../migrations/up/add_images.sql"),
		down: include_str!("../../migrations/down/add_images.sql"),
	},
	References {
		id: 3,
		up: include_str!("../../migrations/up/add_annotations_table.sql"),
		down: include_str!("../../migrations/down/add_annotations_table.sql"),
	},
];
const ADD_MIGRATIONS: &'static str = r#"
	INSERT OR REPLACE INTO _migrations (id, up, down, applied)
	VALUES (?1, ?2, ?3, ?4);
"#;

pub fn migrate(path: &PathBuf) -> anyhow::Result<()> {
	info!("Applying migrations to sqlite db: {}...", path.display());
	let connection = Connection::open(path)?;
	let _ = &connection.execute_batch(INITIALIZE)?;
	let mut migrations: HashMap<u64, Migration> = HashMap::new();
	let mut stmt = connection.prepare("SELECT * FROM _migrations;")?;
	let migs = stmt.query_map([], |row| Migration::from_row(row))?;
	for mig in migs {
		let m = mig?;
		if migrations.contains_key(&m.id) {
			return Err(Error::msg(format!("Two migrations with ID {}", m.id)));
		}
		migrations.insert(m.id, m);
	}
	let mut mig_up_stmt = connection.prepare(ADD_MIGRATIONS)?;
	for refs in REFERENCES {
		let applied_mig = migrations.get(&refs.id);
		if applied_mig.is_none() || applied_mig.is_some_and(|m| !m.applied){
			trace!("Applying migration {}\n\"\"\"\n{}\n\"\"\"\n", refs.id, refs.up);
			connection.execute_batch(refs.up)?;
			trace!("Execute\n\"\"\"\n{}\n\"\"\"\nwith: [{}, {}, {}]", ADD_MIGRATIONS, refs.id, refs.up, refs.down);
			mig_up_stmt.execute(params![refs.id, refs.up, refs.down, true])?;
		}
	}
	// TODO: Add down migs
	mig_up_stmt.finalize()?;
	info!("Successfully applying migrations to sqlite db: {}...", path.display());
	Ok(())
}