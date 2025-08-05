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
            for (id, licence) in dataset.licenses() {
                license_mapper.add_licence(*id, dataset.name().clone(), licence.clone());
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

    pub fn from_storable(
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
        for (group, entries) in datasets.entries() {
            let mut sub_entries = HashMap::new();
            sub_entries.insert(group.clone(), entries.clone());
            let set: DsmSets = DsmSets::new(datasets.metadata().clone(), sub_entries);
            if let Some(ds) = map_set.get_mut(group) {
                ds.push(set);
            } else {
                map_set.insert(group.clone(), vec![set.clone()]);
            }
        }

        Self {
            datasets: map_set,
            name: datasets.name().clone(),
            version: datasets.version().clone(),
            class_mapper,
            date_created: String::default(),
            description: String::default(),
            license_mapper,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn to_dataset(&self, form: DsmDataForm) -> anyhow::Result<DsmSets> {
        let classes = self.class_mapper.get_dsm_classes();
        let metadata =
            DsmMetaDataBuilder::new(self.name.clone(), self.version.clone(), form, classes)
                .set_date_created(self.date_created.clone())
                .set_licenses(self.license_mapper.licences().clone())
                .set_description(self.description.clone())
                .build();
        let mut data_entries: HashMap<String, Vec<DsmEntry>> = HashMap::new();
        for (group, datasets) in &self.datasets {
            if !data_entries.contains_key(group) {
                data_entries.insert(group.to_string(), Vec::new());
            }
            for dataset in datasets {
                for entries in dataset.entries().values() {
                    for entry in entries {
                        let new_entry = entry.remap_with_rename(
                            dataset.name().clone(),
                            dataset.version().clone(),
                            &self.class_mapper,
                            &self.license_mapper,
                            dataset.classes().clone(),
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
        Ok(DsmSets::new(metadata, data_entries))
    }

    pub fn datasets_include(&self) -> &HashMap<String, Vec<DsmSets>> {
        &self.datasets
    }

    pub fn class_mapping(&self) -> &ClassMapper {
        &self.class_mapper
    }

    pub fn license_mapping(&self) -> &LicenseMapper {
        &self.license_mapper
    }

    pub fn date_created(&self) -> &String {
        &self.date_created
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn to_storable(&self) -> StorableMerged {
        let mut sets = HashMap::new();

        for (group, datasets) in &self.datasets {
            for dataset in datasets {
                sets.insert(
                    format!("{}~{}", dataset.name(), dataset.version()),
                    GroupSet::local(group.clone()),
                );
            }
        }

        StorableMerged::new(
            self.name.clone(),
            self.version.clone(),
            sets,
            self.class_mapper.clone(),
            self.license_mapper.clone(),
        )
    }
}
