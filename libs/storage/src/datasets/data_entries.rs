use log::trace;
use interfaces::models::entries::DatasetEntry;
use rusqlite::{params, Connection};

const INSERT_NEW_IMAGE: &'static str = "INSERT INTO images(names, width, height, paths) VALUES(?1, ?2, ?3, ?4) RETURNING ROWID;";
const INSERT_NEW_IMAGE_VERSION: &'static str = "INSERT INTO version_images(version_id, image_id) VALUES(?1, ?2) RETURNING ROWID;";
const INSERT_NEW_ANNOTATION: &'static str = r#"
	INSERT INTO annotations(version_image_id, class, x, y, width, height)
	VALUES(?1, ?2, ?3, ?4, ?5, ?6);
"#;

pub(crate) fn store(connection: &Connection, version_id: i32, entry: &DatasetEntry) -> anyhow::Result<()> {
	trace!("{}", INSERT_NEW_IMAGE);
	let mut stmt = connection.prepare(INSERT_NEW_IMAGE)?;
	let image_id : u32 = stmt.query_row(params![
		&entry.get_image_path().file_name().unwrap().to_str().unwrap(),
		&entry.get_width(),
		&entry.get_height(),
		&version_id
	], |r| r.get(0))?;
	stmt.finalize()?;

	trace!("{}", INSERT_NEW_IMAGE_VERSION);
	let mut stmt = connection.prepare(INSERT_NEW_IMAGE_VERSION)?;
	let version_images_id : u32 = stmt.query_row(params![&version_id, &image_id], |r| r.get(0))?;
	stmt.finalize()?;

	trace!("{}", INSERT_NEW_ANNOTATION);
	let mut stmt = connection.prepare(INSERT_NEW_ANNOTATION)?;
	for annotation in entry.get_annotation() {
		stmt.execute(params![
			version_images_id,
			annotation.get_class(),
			annotation.get_x(),
			annotation.get_y(),
			annotation.get_width(),
			annotation.get_height()
		])?;
	}
	stmt.finalize()?;
	Ok(())
}