use interfaces::models::annotation::Annotation;
use interfaces::models::entries::DatasetEntry;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::path::PathBuf;

const INSERT_NEW_IMAGE: &'static str = "INSERT INTO images(names, width, height, paths) VALUES(?1, ?2, ?3, ?4) RETURNING ROWID;";
const INSERT_NEW_IMAGE_VERSION: &'static str = "INSERT INTO version_images(version_id, image_id) VALUES(?1, ?2) RETURNING ROWID;";
const INSERT_NEW_ANNOTATION: &'static str = r#"
	INSERT INTO annotations(version_image_id, class_id, x, y, width, height)
	VALUES(?1, ?2, ?3, ?4, ?5, ?6);
"#;

pub(crate) fn store(connection: &Connection, version_id: i32, entry: &DatasetEntry, class_mapping: &HashMap<u32, i32>) -> anyhow::Result<()> {
	let mut stmt = connection.prepare(INSERT_NEW_IMAGE)?;
	let image_id : u32 = stmt.query_row(params![
		&entry.get_image_path().file_name().unwrap().to_str().unwrap(),
		&entry.get_width(),
		&entry.get_height(),
		&entry.get_image_path().to_string_lossy().to_string()
	], |r| r.get(0))?;
	stmt.finalize()?;

	let mut stmt = connection.prepare(INSERT_NEW_IMAGE_VERSION)?;
	let version_images_id : u32 = stmt.query_row(params![&version_id, &image_id], |r| r.get(0))?;
	stmt.finalize()?;

	let mut stmt = connection.prepare(INSERT_NEW_ANNOTATION)?;
	for annotation in entry.get_annotation() {
		stmt.execute(params![
			version_images_id,
			class_mapping.get(&annotation.get_class()).expect("Class not found"),
			annotation.get_x(),
			annotation.get_y(),
			annotation.get_width(),
			annotation.get_height()
		])?;
	}
	stmt.finalize()?;
	Ok(())
}

pub(crate) fn read(connection: &Connection, version_id: i32, mapping: &HashMap<i32, u32>) -> anyhow::Result<Vec<DatasetEntry>> {
	let mut stmt = connection.prepare(r#"
		SELECT version_images.id, annotations.class_id, annotations.x, annotations.y, annotations.width, annotations.height
		FROM version_images
		INNER JOIN annotations on version_images.id = annotations.version_image_id
		WHERE version_id = ?1;
	"#)?;
	let annotation = stmt.query_map(params![&version_id], |r| {
		let version_image_id :u32 = r.get(0)?;
		let class: i32 = r.get(1)?;
		let x = r.get(2)?;
		let y = r.get(3)?;
		let width = r.get(4)?;
		let height = r.get(5)?;
		return Ok((version_image_id, Annotation::new(*mapping.get(&class).expect("Class not found"), x, y, width, height)));
	})?;
	let mut annotation_mapping: HashMap<u32, Vec<Annotation>> = HashMap::new();
	for ann in annotation {
		let (id, annot) = ann?;
		if let Some(annotation) = annotation_mapping.get_mut(&id) {
			annotation.push(annot)
		} else {
			annotation_mapping.insert(id, vec![annot]);
		}
	}

	let mut stmt = connection.prepare(r#"
		SELECT version_images.id, images.width, images.height, images.paths
		FROM version_images
		INNER JOIN images on images.id = version_images.image_id
		WHERE version_id = ?1;
	"#)?;
	let images = stmt.query_map(params![&version_id], |r| {
		let id: u32 = r.get(0)?;
		let width: u32 = r.get(1)?;
		let height: u32 = r.get(2)?;
		let paths: String = r.get(3)?;
		return Ok((id, width, height, paths))
	})?;
	let mut entries = Vec::new();
	for img in images {
		let (id, width, height, paths) = img?;
		if let Some(annotations) = annotation_mapping.get(&id) {
			entries.push(DatasetEntry::new(PathBuf::from(paths), width, height, annotations.clone()));
		} else {
			entries.push(DatasetEntry::new(PathBuf::from(paths), width, height, Vec::new()));
		}
	}
	Ok(entries)
}