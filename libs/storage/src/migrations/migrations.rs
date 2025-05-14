use rusqlite::Row;

#[derive(Clone, Debug)]
pub struct Migration {
	pub id: u64,
	pub up: String,
	pub down: String,
	pub applied: bool
}

impl Migration {
	pub(crate) fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
		let id: u64 = row.get(0)?;
		let up: String = row.get(1)?;
		let down: String = row.get(2)?;
		let applied: bool = row.get(3)?;
		Ok(Self {
			id: id.clone(),
			up: up.clone(),
			down: down.clone(),
			applied: applied.clone()
		})
	}
}