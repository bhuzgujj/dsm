use crate::log_err;
use crate::models::entries::DsmEntry;
use crate::models::metadata::DsmMetaDataBuilder;
use crate::models::storable_merged::StorableMerged;
use crate::models::{ClassMapper, DsmDataForm, DsmSets, GroupSet, LicenseMapper};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct MergedSet {
    datasets: HashMap<String, Vec<DsmSets>>,
    name: String,
    version: String,
    class_mapper: ClassMapper,
    license_mapper: LicenseMapper,
    date_created: String,
    description: String,
}

impl MergedSet {
    pub fn new(
        datasets: HashMap<String, (String, DsmSets)>,
        name: String,
        version: String,
        class_mapper: ClassMapper,
    ) -> Self {
        let sets: Vec<(String, DsmSets)> = datasets.values().cloned().collect();
        let mut datasets: HashMap<String, Vec<DsmSets>> = HashMap::new();
        let mut license_mapper = LicenseMapper::new();
        for (group, dataset) in &sets {
            for (id, licence) in dataset.get_license() {
                license_mapper.add_licence(*id, dataset.get_name().clone(), licence.clone());
            }
            if let Some(ds) = datasets.get_mut(group) {
                ds.push(dataset.clone());
            } else {
                datasets.insert(group.clone(), vec![dataset.clone()]);
            }
        }
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

    pub fn from_vec(
        datasets: Vec<(String, DsmSets)>,
        name: String,
        version: String,
        class_mapper: ClassMapper,
        license_mapper: LicenseMapper,
    ) -> Self {
        let mut map_set: HashMap<String, Vec<DsmSets>> = HashMap::new();
        for (group, set) in datasets {
            if let Some(ds) = map_set.get_mut(&group) {
                ds.push(set.clone());
            } else {
                map_set.insert(group, vec![set.clone()]);
            }
        }

        Self {
            datasets: map_set,
            name,
            version,
            class_mapper,
            date_created: String::default(),
            description: String::default(),
            license_mapper,
        }
    }

    pub fn from_dsm(
        datasets: DsmSets,
        class_mapper: ClassMapper,
        license_mapper: LicenseMapper,
    ) -> Self {
        let mut map_set: HashMap<String, Vec<DsmSets>> = HashMap::new();
        for (group, entries) in datasets.get_entries() {
            let mut sub_entries = HashMap::new();
            sub_entries.insert(group.clone(), entries.clone());
            let set: DsmSets = DsmSets::new(datasets.get_metadata().clone(), sub_entries);
            if let Some(ds) = map_set.get_mut(group) {
                ds.push(set);
            } else {
                map_set.insert(group.clone(), vec![set.clone()]);
            }
        }

        Self {
            datasets: map_set,
            name: datasets.get_name().clone(),
            version: datasets.get_version().clone(),
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
        for (group, datasets) in &self.datasets {
            if !data_entries.contains_key(group) {
                data_entries.insert(group.to_string(), Vec::new());
            }
            for dataset in datasets {
                for (_, entries) in dataset.get_entries() {
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
                        if let Some(entries) = data_entries.get_mut(group) {
                            entries.push(new_entry);
                        } else {
                            return log_err!(format!(
                                "Entries vector for '{group}' has not been inserted"
                            ));
                        }
                    }
                }
            }
        }
        Ok((DsmSets::new(metadata, data_entries), Some(rel_path)))
    }

    pub fn get_datasets_include(&self) -> &HashMap<String, Vec<DsmSets>> {
        &self.datasets
    }

    pub fn get_class_mapping(&self) -> &ClassMapper {
        &self.class_mapper
    }

    pub fn get_license_mapping(&self) -> &LicenseMapper {
        &self.license_mapper
    }

    pub fn get_date_created(&self) -> &String {
        &self.date_created
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn to_storable(&self) -> StorableMerged {
        let mut sets = HashMap::new();

        for (group, datasets) in &self.datasets {
            for dataset in datasets {
                sets.insert(
                    format!("{}~{}", dataset.get_name(), dataset.get_version()),
                    GroupSet {
                        location: dataset.get_metadata().get_location().clone(),
                        group: group.clone()
                    },
                );
            }
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
