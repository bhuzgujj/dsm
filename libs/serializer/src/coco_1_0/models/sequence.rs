use crate::coco_1_0::models::annotation::Annotation;
use crate::coco_1_0::models::category::Category;
use crate::coco_1_0::models::image::Image;
use crate::coco_1_0::models::info::Info;
use crate::coco_1_0::models::license::License;
use interfaces::log_err;
use interfaces::models::entries::{DsmEntry, DsmEntryBuilder};
use interfaces::models::metadata::{DsmMetaData, DsmMetaDataBuilder};
use interfaces::models::DsmSets;
use interfaces::models::{DsmDataForm, DsmLocation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) const IMAGE_PATH: &str = "images";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Sequence {
	pub(crate) licenses: Vec<License>,
	pub(crate) info: Info,
	pub(crate) categories: Vec<Category>,
	pub(crate) images: Vec<Image>,
	pub(crate) annotations: Vec<Annotation>,
}

impl Sequence {
	pub(crate) fn from_dsm(datasets: &DsmSets) -> HashMap<String, Self> {
		let mut sequences = HashMap::new();
		let mut images_index: u32 = 0;
		let mut annotations_index: u32 = 0;
		for (subset, entries) in datasets.entries() {
			let (sequence, imgi, anni) = Self::from_parts(
				datasets.metadata(),
				entries.clone(),
				&images_index,
				&annotations_index,
			);
			images_index = imgi;
			annotations_index = anni;
			sequences.insert(subset.clone(), sequence);
		}

		sequences
	}

	fn from_parts(
		meta_data: &DsmMetaData,
		entries: Vec<DsmEntry>,
		images_i: &u32,
		annotations_i: &u32,
	) -> (Self, u32, u32) {
		let licenses: Vec<License> =
			meta_data
				.licenses()
				.clone()
				.iter()
				.fold(Vec::new(), |mut acc, (index, license)| {
					acc.push(License::from_dsm(index, license));
					acc
				});
		let mut images_index = *images_i;
		let mut annotations_index = *annotations_i;
		let info: Info = Info::from_dsm(meta_data);
		let categories: Vec<Category> =
			meta_data
				.classes()
				.iter()
				.fold(Vec::new(), |mut acc, (index, class)| {
					acc.push(Category::from_dsm(class, *index));
					acc
				});
		let mut images: Vec<Image> = Vec::with_capacity(entries.len());
		let mut annotations: Vec<Annotation> = Vec::with_capacity(entries.len());
		for entry in entries {
			images_index += 1;
			images.push(Image::from_dsm(images_index, &entry));
			for annotation in entry.annotation() {
				annotations_index += 1;
				annotations.push(Annotation::from_dsm(
					annotations_index,
					images_index,
					annotation,
				));
			}
		}

		(
			Self {
				licenses,
				info,
				categories,
				images,
				annotations,
			},
			images_index,
			annotations_index,
		)
	}
}

pub fn into_dsm(
	sequences: HashMap<String, Sequence>,
	data_form: &DsmDataForm,
	name: &str,
	version: &str,
	root: &Path,
) -> anyhow::Result<DsmSets> {
	let mut entries = HashMap::new();
	let mut categories: Vec<Category> = Vec::new();
	let mut licenses: Vec<License> = Vec::new();
	let mut contributor: Option<String> = None;
	let mut date_created: Option<String> = None;
	let mut url: Option<String> = None;
	let mut year: Option<String> = None;
	let mut description: Option<String> = None;

	for (json, sequence) in sequences {
		let group = strip_name(json)?;

		// TODO: Try to keep the information in both dsm and merged
		if contributor.is_none() {
			contributor = Some(sequence.info.contributor.clone());
		}
		if date_created.is_none() {
			date_created = Some(sequence.info.date_created.clone());
		}
		if url.is_none() {
			url = Some(sequence.info.url.clone());
		}
		if year.is_none() {
			year = Some(sequence.info.year.clone());
		}
		if description.is_none() {
			description = Some(sequence.info.description.clone());
		}
		entries.insert(group.clone(), into_group_entries(&sequence, &group, root));
		for licence in sequence.licenses {
			if !licenses.contains(&licence) {
				licenses.push(licence.clone());
			}
		}
		for category in sequence.categories {
			if !categories.contains(&category) {
				categories.push(category.clone());
			}
		}
	}

	let mut metadata = DsmMetaDataBuilder::new(
		name.to_string(),
		version.to_string(),
		data_form.clone(),
		categories.iter().fold(HashMap::new(), |mut acc, category| {
			let _ = acc.insert(category.id, category.dsm());
			acc
		}),
	)
	.set_licenses(licenses.iter().fold(HashMap::new(), |mut acc, license| {
		let _ = acc.insert(license.id, license.dsm());
		acc
	}));
	if let Some(contrib) = contributor {
		metadata = metadata.set_contributor(contrib);
	}
	if let Some(date) = date_created {
		metadata = metadata.set_date_created(date);
	}
	if let Some(u) = url {
		metadata = metadata.set_url(u);
	}
	if let Some(y) = year {
		metadata = metadata.set_year(y);
	}
	if let Some(desc) = description {
		metadata = metadata.set_description(desc);
	}
	return Ok(DsmSets::new(metadata.build(), entries));
}

fn into_group_entries(sequences: &Sequence, group: &String, root: &Path) -> Vec<DsmEntry> {
	let mut annotations: HashMap<u32, Vec<interfaces::models::annotation::DsmAnnotation>> =
		HashMap::new();
	for ann in &sequences.annotations {
		if let std::collections::hash_map::Entry::Vacant(e) = annotations.entry(ann.image_id) {
			e.insert(vec![ann.dsm()]);
		} else {
			let img = annotations
				.get_mut(&ann.image_id)
				.expect("ann.id not found in annotations");
			img.push(ann.dsm());
		}
	}
	let mut data_entries = Vec::new();
	let path = PathBuf::from(&IMAGE_PATH).join(group.clone());
	for imgs in &sequences.images {
		data_entries.push(
			DsmEntryBuilder::new(
				imgs.file_name.clone(),
				path.join(&imgs.file_name).clone(),
				imgs.width,
				imgs.height,
				DsmLocation::Local {
					path: root
						.to_str()
						.expect("Could not stringify the rootpath?")
						.to_string(),
				},
			)
			.set_license(Some(imgs.license))
			.set_annotation(annotations.get(&imgs.id).unwrap_or(&Vec::new()).clone())
			.set_coco_url(imgs.coco_url.clone())
			.set_date_captured(Some(imgs.date_captured.clone()))
			.set_flickr_url(imgs.flickr_url.clone())
			.build(),
		)
	}
	return data_entries;
}

fn strip_name(json_name: String) -> anyhow::Result<String> {
	let parts: Vec<&str> = json_name.split(".").collect();
	if parts.len() != 2 {
		return log_err!(format!("'{json_name}' is not a valid json name"));
	}
	Ok(parts[0].split('_').next_back().unwrap().to_string())
}
