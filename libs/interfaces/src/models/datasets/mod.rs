use entries::DatasetEntry;
use metadata::MetaData;

pub mod entries;
pub mod metadata;

pub struct Dataset {
    metadata: MetaData,
    entries: Vec<DatasetEntry>,
}
