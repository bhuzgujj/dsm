use crate::coco_1_0::models::annotation::Annotation;
use crate::coco_1_0::models::category::Category;
use crate::coco_1_0::models::image::Image;
use crate::coco_1_0::models::info::Info;
use crate::coco_1_0::models::license::License;
use anyhow::anyhow;
use interfaces::models::entries::DatasetEntry;
use interfaces::models::metadata::MetaData;
use interfaces::models::DataForm;
use interfaces::models::Datasets;
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
    pub(crate) fn to_datasets(self, data_form: &DataForm, name: &String, version: &Option<u32>, subset: String) -> anyhow::Result<Datasets> {
        let actual_name = strip_name(subset)?;
        let version = version.unwrap_or(1);
        let n = format!("{}-{}", name.clone(), actual_name.clone());
        let metadata: MetaData = MetaData::new(
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
        let mut entries: HashMap<String, Vec<DatasetEntry>> = HashMap::new();
        let mut annotations: HashMap<u32, Vec<interfaces::models::annotation::Annotation>> = HashMap::new();
        for ann in self.annotations {
            if annotations.contains_key(&ann.image_id) {
                let img = annotations
                    .get_mut(&ann.image_id)
                    .expect("ann.id not found in annotations");
                img.push(ann.to_datasettable());
            } else {
                annotations.insert(ann.image_id, vec![ann.to_datasettable()]);
            }
        }
        let mut data_entries = Vec::new();
        let path = PathBuf::from(&n).join(format!("v{}", version)).join(IMAGE_PATH).join(actual_name.clone());
        for imgs in self.images {
            data_entries.push(DatasetEntry::new(
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

        Ok(Datasets::new(metadata, entries))
    }

    pub(crate) fn from_datasets(datasets: &Datasets) -> anyhow::Result<(HashMap<String, Self>, HashMap<String, String>)> {
        let mut sequences = HashMap::new();
        let mut image_map = HashMap::new();
        for (subset, entries) in datasets.get_entries() {
            let (sequence, images) = Self::from_parts(datasets.get_metadata(), entries.clone())?;
            for (refs, name) in &images {
                image_map.insert(name.clone(), refs.clone());
            }
            sequences.insert(subset.clone(), sequence);
        }

        Ok((sequences, image_map))
    }

    fn from_parts(meta_data: &MetaData, entries: Vec<DatasetEntry>) -> anyhow::Result<(Self, HashMap<String, String>)> {
        let mut image_map = HashMap::new();
        let mut licenses: Vec<License>,
        let mut info: Info = Info::from_meta(meta_data);
        let mut categories: Vec<Category> = meta_data.get_classes().iter().fold(Vec::new(), |mut acc, (index, class)| {
            acc.push(Category::from_classes(class, *index));
            acc
        });
        let mut images: Vec<Image> = Vec::with_capacity(entries.len());
        let mut annotations: Vec<Annotation> = Vec::with_capacity(entries.len());
        for entry in entries {

        }

        Ok((Self {
            licenses,
            info,
            categories,
            images,
            annotations
        }, image_map))
    }
}

fn strip_name(json_name: String) -> anyhow::Result<String> {
    let parts: Vec<&str> = json_name.split(".").collect();
    if parts.len() != 2 {
        return Err(anyhow!("'{json_name}' is not a valid json name"));
    }
    Ok(parts[0].split('_').last().unwrap().to_string())
}
