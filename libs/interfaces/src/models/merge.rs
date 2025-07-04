use crate::log_err;
use crate::models::entries::DsmEntry;
use crate::models::metadata::DsmMetaDataBuilder;
use crate::models::storable_merged::StorableMerged;
use crate::models::{ClassMapper, DsmDataForm, DsmSets, LicenseMapper};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct MergedSet {
    datasets: Vec<DsmSets>,
    name: String,
    version: String,
    class_mapper: ClassMapper,
    license_mapper: LicenseMapper,
    date_created: String,
    description: String,
}

impl MergedSet {
    pub fn new(
        datasets: HashMap<String, DsmSets>,
        name: String,
        version: String,
        class_mapper: ClassMapper,
    ) -> Self {
        let sets: Vec<DsmSets> = datasets.values().cloned().collect();
        let mut license_mapper = LicenseMapper::new();
        for dataset in &sets {
            for (id, licence) in dataset.get_license() {
                license_mapper.add_licence(*id, dataset.get_name().clone(), licence.clone());
            }
        }
        Self {
            datasets: sets,
            name,
            version,
            class_mapper,
            date_created: String::default(),
            description: String::default(),
            license_mapper,
        }
    }

    pub fn from_vec(
        datasets: Vec<DsmSets>,
        name: String,
        version: String,
        class_mapper: ClassMapper,
        license_mapper: LicenseMapper,
    ) -> Self {
        Self {
            datasets,
            name,
            version,
            class_mapper,
            date_created: String::default(),
            description: String::default(),
            license_mapper,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn to_dataset(
        &self,
        form: DsmDataForm,
    ) -> anyhow::Result<(DsmSets, Option<HashMap<String, String>>)> {
        let classes = self.class_mapper.get_dsm_classes();
        let metadata =
            DsmMetaDataBuilder::new(self.name.clone(), self.version.clone(), form, classes)
                .set_date_created(self.date_created.clone())
                .set_licenses(self.license_mapper.get_licences().clone())
                .set_description(self.description.clone())
                .build();
        let mut data_entries: HashMap<String, Vec<DsmEntry>> = HashMap::new();
        let mut rel_path: HashMap<String, String> = HashMap::new();
        for dataset in &self.datasets {
            for (name, entries) in dataset.get_entries() {
                if !data_entries.contains_key(name) {
                    data_entries.insert(name.to_string(), Vec::new());
                }
                for entry in entries {
                    let new_entry = entry.map_in(
                        dataset.get_name().clone(),
                        dataset.get_version().clone(),
                        &self.class_mapper,
                        &self.license_mapper,
                        dataset.get_classes().clone(),
                    );
                    rel_path.insert(
                        new_entry.get_file_name().clone(),
                        dataset.get_rel_from_storage().clone(),
                    );
                    if let Some(entries) = data_entries.get_mut(name) {
                        entries.push(new_entry);
                    } else {
                        return log_err!(format!(
                            "Entries vector for '{name}' has not been inserted"
                        ));
                    }
                }
            }
        }
        Ok((DsmSets::new(metadata, data_entries), Some(rel_path)))
    }

    pub fn get_datasets_include(&self) -> &Vec<DsmSets> {
        &self.datasets
    }

    pub fn get_mapping(&self) -> &ClassMapper {
        &self.class_mapper
    }

    pub fn to_storable(&self) -> StorableMerged {
        let mut sets = HashMap::new();

        for dataset in &self.datasets {
            sets.insert(
                format!("{}~{}", dataset.get_name(), dataset.get_version()),
                dataset.get_metadata().get_location().clone(),
            );
        }

        StorableMerged {
            name: self.name.clone(),
            version: self.version.clone(),
            datasets: sets,
            class_mapper: self.class_mapper.clone(),
            license_mapper: self.license_mapper.clone(),
        }
    }
}
