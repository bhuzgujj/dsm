use crate::coco_1_0::models::annotation::Annotation;
use crate::coco_1_0::models::category::Category;
use crate::coco_1_0::models::image::Image;
use crate::coco_1_0::models::info::Info;
use crate::coco_1_0::models::license::License;
use anyhow::anyhow;
use interfaces::models::entries::DsmEntry;
use interfaces::models::metadata::DsmMetaData;
use interfaces::models::DataForm;
use interfaces::models::DsmSets;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
    pub(crate) fn to_datasets(self, data_form: &DataForm, name: &String, version: &Option<u32>, subset: String) -> anyhow::Result<DsmSets> {
        let actual_name = strip_name(subset)?;
        let version = version.unwrap_or(1);
        let n = format!("{}-{}", name.clone(), actual_name.clone());
        let metadata: DsmMetaData = DsmMetaData::new(
            n.clone(),
            version,
            Some(self.info.version),
            self.info.contributor,
            self.info.date_created,
            self.info.description,
            self.info.url,
            self.info.year,
            data_form.clone(),
            self.categories
                .iter()
                .fold(HashMap::new(), |mut acc, category| {
                    let _ = acc.insert(category.id, category.to_classes());
                    acc
                }),
            self.licenses
                .iter()
                .fold(HashMap::new(), |mut acc, license| {
                    let _ = acc.insert(license.id, license.to_datasettable());
                    acc
                }),
        );
        let mut entries: HashMap<String, Vec<DsmEntry>> = HashMap::new();
        let mut annotations: HashMap<u32, Vec<interfaces::models::annotation::DsmAnnotation>> = HashMap::new();
        for ann in self.annotations {
            if let std::collections::hash_map::Entry::Vacant(e) = annotations.entry(ann.image_id) {
                e.insert(vec![ann.to_datasettable()]);
            } else {
                let img = annotations
                    .get_mut(&ann.image_id)
                    .expect("ann.id not found in annotations");
                img.push(ann.to_datasettable());
            }
        }
        let mut data_entries = Vec::new();
        let path = PathBuf::from(&n).join(format!("v{}", version)).join(IMAGE_PATH).join(actual_name.clone());
        for imgs in self.images {
            data_entries.push(DsmEntry::new(
                path.join(&imgs.file_name).clone(),
                imgs.width,
                imgs.height,
                imgs.file_name.clone(),
                Some(imgs.license),
                Some(imgs.flickr_url),
                Some(imgs.coco_url),
                Some(imgs.date_captured),
                annotations
	                .get(&imgs.id)
	                .unwrap_or(&Vec::new())
	                .clone()
            ))
        }
        entries.insert(actual_name.clone(), data_entries);

        Ok(DsmSets::new(metadata, entries))
    }

    pub(crate) fn from_datasets(datasets: &DsmSets) -> anyhow::Result<(HashMap<String, Self>, HashMap<String, PathBuf>)> {
        let mut sequences = HashMap::new();
        let mut image_map: HashMap<String, PathBuf> = HashMap::new();
        let mut images_index: u32 = 0;
        let mut annotations_index: u32 = 0;
        for (subset, entries) in datasets.get_entries() {
            let (sequence, images, imgi, anni) = Self::from_parts(
                datasets.get_metadata(),
                entries.clone(),
                &images_index,
                &annotations_index
            )?;
            images_index = imgi;
            annotations_index = anni;
            for (name, refs) in &images {
                image_map.insert(name.clone(), refs.clone());
            }
            sequences.insert(subset.clone(), sequence);
        }

        Ok((sequences, image_map))
    }

    fn from_parts(meta_data: &DsmMetaData, entries: Vec<DsmEntry>, images_i: &u32, annotations_i: &u32) -> anyhow::Result<(Self, HashMap<String, PathBuf>, u32, u32)> {
        let mut image_map = HashMap::new();
        let licenses: Vec<License> = meta_data.get_licenses().clone().iter().fold(Vec::new(), |mut acc, (index, license)| {
            acc.push(License::from_base(index, license));
            acc
        });
        let mut images_index = *images_i;
        let mut annotations_index = *annotations_i;
        let info: Info = Info::from_meta(meta_data);
        let categories: Vec<Category> = meta_data.get_classes().iter().fold(Vec::new(), |mut acc, (index, class)| {
            acc.push(Category::from_classes(class, *index));
            acc
        });
        let mut images: Vec<Image> = Vec::with_capacity(entries.len());
        let mut annotations: Vec<Annotation> = Vec::with_capacity(entries.len());
        for entry in entries {
            images_index += 1;
            images.push(Image::from_data_entry(images_index, &entry));
            for annotation in entry.get_annotation() {
                annotations_index += 1;
                annotations.push(Annotation::from_interface(annotations_index, images_index, annotation));
            }
            image_map.insert(entry.get_file_name(), entry.get_image_relative_path());
        }

        Ok((Self {
            licenses,
            info,
            categories,
            images,
            annotations
        }, image_map, images_index, annotations_index))
    }
}

fn strip_name(json_name: String) -> anyhow::Result<String> {
    let parts: Vec<&str> = json_name.split(".").collect();
    if parts.len() != 2 {
        return Err(anyhow!("'{json_name}' is not a valid json name"));
    }
    Ok(parts[0].split('_').next_back().unwrap().to_string())
}
