use interfaces::models::{ClassMapper, DsmSets, LicenseMapper, MergedSet};

pub trait Storable {
    fn from_dsm(dataset: DsmSets) -> Self;
    fn to_dsm(dataset: Self) -> anyhow::Result<DsmSets>;
}

impl Storable for MergedSet {
    fn from_dsm(dataset: DsmSets) -> Self {
        let name = dataset.get_name().clone();
        let version = dataset.get_version().clone();
        let c_mapping = ClassMapper::from(&dataset);
        let l_mapping = LicenseMapper::from(&dataset);
        MergedSet::from_vec(vec![dataset], name, version, c_mapping, l_mapping)
    }

    fn to_dsm(dataset: Self) -> anyhow::Result<DsmSets> {
        Ok(dataset
            .to_dataset(interfaces::models::DsmDataForm::Yolo1_1)?
            .0)
    }
}

impl Storable for DsmSets {
    fn from_dsm(dataset: DsmSets) -> Self {
        dataset
    }

    fn to_dsm(dataset: Self) -> anyhow::Result<DsmSets> {
        Ok(dataset)
    }
}
