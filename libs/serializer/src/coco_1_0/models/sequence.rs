use crate::coco_1_0::models::annotation::CocoAnnotation;
use crate::coco_1_0::models::category::CocoCategory;
use crate::coco_1_0::models::image::CocoImage;
use crate::coco_1_0::models::info::CocoInfo;
use crate::coco_1_0::models::license::CocoLicense;
use interfaces::log_err;
use interfaces::models::datasets::{Annotation, Dataset};
use interfaces::models::datasets::{Entry, EntryBuilder};
use interfaces::models::datasets::{MetaData, MetaDataBuilder};
use interfaces::models::{DataFormat, Location};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) const IMAGE_PATH: &str = "images";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CocoSequence {
	pub(crate) licenses: Vec<CocoLicense>,
	pub(crate) info: CocoInfo,
	pub(crate) categories: Vec<CocoCategory>,
	pub(crate) images: Vec<CocoImage>,
	pub(crate) annotations: Vec<CocoAnnotation>,
}

impl CocoSequence {
	pub(crate) fn from_dsm(datasets: &Dataset) -> HashMap<String, Self> {
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
		meta_data: &MetaData,
		entries: Vec<Entry>,
		images_i: &u32,
		annotations_i: &u32,
	) -> (Self, u32, u32) {
		let licenses: Vec<CocoLicense> =
			meta_data
				.licenses()
				.clone()
				.iter()
				.fold(Vec::new(), |mut acc, (index, license)| {
					acc.push(CocoLicense::from_dsm(index, license));
					acc
				});
		let mut images_index = *images_i;
		let mut annotations_index = *annotations_i;
		let info: CocoInfo = CocoInfo::from_dsm(meta_data);
		let categories: Vec<CocoCategory> =
			meta_data
				.classes()
				.iter()
				.fold(Vec::new(), |mut acc, (index, class)| {
					acc.push(CocoCategory::from_dsm(class, *index));
					acc
				});
		let mut images: Vec<CocoImage> = Vec::with_capacity(entries.len());
		let mut annotations: Vec<CocoAnnotation> = Vec::with_capacity(entries.len());
		for entry in entries {
			images_index += 1;
			images.push(CocoImage::from_dsm(images_index, &entry));
			for annotation in entry.annotation() {
				annotations_index += 1;
				annotations.push(CocoAnnotation::from_dsm(
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
	sequences: HashMap<String, CocoSequence>,
	data_form: &DataFormat,
	name: &str,
	version: &str,
	root: &Path,
) -> anyhow::Result<Dataset> {
	let mut entries = HashMap::new();
	let mut categories: Vec<CocoCategory> = Vec::new();
	let mut licenses: Vec<CocoLicense> = Vec::new();
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

	let mut metadata = MetaDataBuilder::new(
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
	Ok(Dataset::new(metadata.build(), entries))
}

fn into_group_entries(sequences: &CocoSequence, group: &str, root: &Path) -> Vec<Entry> {
	let mut annotations: HashMap<u32, Vec<Annotation>> = HashMap::new();
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
	let path = PathBuf::from(&IMAGE_PATH).join(group);
	for imgs in &sequences.images {
		data_entries.push(
			EntryBuilder::new(
				imgs.file_name.clone(),
				path.join(&imgs.file_name).clone(),
				imgs.width,
				imgs.height,
				Location::Local {
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
	data_entries
}

fn strip_name(json_name: String) -> anyhow::Result<String> {
	let parts: Vec<&str> = json_name.split(".").collect();
	if parts.len() != 2 {
		return log_err!(format!("'{json_name}' is not a valid json name"));
	}
	Ok(parts[0].split('_').next_back().unwrap().to_string())
}
