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
    pub(crate) fn dsm(
        self,
        data_form: &DsmDataForm,
        name: &str,
        version: &str,
        subset: String,
        root: &Path,
    ) -> anyhow::Result<DsmSets> {
        let actual_name = strip_name(subset)?;
        let new_name = format!("{}-{}", name, actual_name.clone());
        let metadata: DsmMetaData = DsmMetaDataBuilder::new(
            new_name.clone(),
            version.to_owned(),
            data_form.clone(),
            self.categories
                .iter()
                .fold(HashMap::new(), |mut acc, category| {
                    let _ = acc.insert(category.id, category.dsm());
                    acc
                }),
        )
        .set_contributor(self.info.contributor)
        .set_date_created(self.info.date_created)
        .set_licenses(
            self.licenses
                .iter()
                .fold(HashMap::new(), |mut acc, license| {
                    let _ = acc.insert(license.id, license.dsm());
                    acc
                }),
        )
        .set_url(self.info.url)
        .set_year(self.info.year)
        .set_description(self.info.description)
        .build();
        let mut entries: HashMap<String, Vec<DsmEntry>> = HashMap::new();
        let mut annotations: HashMap<u32, Vec<interfaces::models::annotation::DsmAnnotation>> =
            HashMap::new();
        for ann in self.annotations {
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
        let path = PathBuf::from(&IMAGE_PATH).join(actual_name.clone());
        for imgs in self.images {
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
                .set_coco_url(imgs.coco_url)
                .set_date_captured(Some(imgs.date_captured))
                .set_flickr_url(imgs.flickr_url)
                .build(),
            )
        }
        entries.insert(actual_name.clone(), data_entries);

        Ok(DsmSets::new(metadata, entries))
    }

    pub(crate) fn from_dsm(datasets: &DsmSets) -> HashMap<String, Self> {
        let mut sequences = HashMap::new();
        let mut images_index: u32 = 0;
        let mut annotations_index: u32 = 0;
        for (subset, entries) in datasets.get_entries() {
            let (sequence, imgi, anni) = Self::from_parts(
                datasets.get_metadata(),
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
        let licenses: Vec<License> = meta_data.get_licenses().clone().iter().fold(
            Vec::new(),
            |mut acc, (index, license)| {
                acc.push(License::from_dsm(index, license));
                acc
            },
        );
        let mut images_index = *images_i;
        let mut annotations_index = *annotations_i;
        let info: Info = Info::from_dsm(meta_data);
        let categories: Vec<Category> =
            meta_data
                .get_classes()
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
            for annotation in entry.get_annotation() {
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

fn strip_name(json_name: String) -> anyhow::Result<String> {
    let parts: Vec<&str> = json_name.split(".").collect();
    if parts.len() != 2 {
        return log_err!(format!("'{json_name}' is not a valid json name"));
    }
    Ok(parts[0].split('_').next_back().unwrap().to_string())
}
