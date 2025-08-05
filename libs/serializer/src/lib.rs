use interfaces::log_err;
use interfaces::models::interpreter::Interpreter;
use interfaces::models::{DsmDataForm, DsmSets};
use log::debug;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;

mod coco_1_0;
mod yolo_1_1;

#[derive(Debug, Clone)]
pub enum DataForm {
    Yolo1_1,
    Coco1_0,
    Custom(String),
}

impl Display for DataForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataForm::Yolo1_1 => write!(f, "Yolo 1.1"),
            DataForm::Coco1_0 => write!(f, "Coco 1.0"),
            DataForm::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

impl DataForm {
    pub fn read(
        &self,
        path: &Path,
        name: Option<String>,
        version: String,
        interpreters: &HashMap<String, Interpreter>,
    ) -> anyhow::Result<Vec<DsmSets>> {
        debug!(
            "Reading files format '{}' at '{}'",
            &path.canonicalize()?.to_str().unwrap_or("<Unknown path>"),
            &self
        );
        match &self {
            DataForm::Yolo1_1 => yolo_1_1::read(path, name, version, self.clone().into()),
            DataForm::Coco1_0 => coco_1_0::read(path, name, version, self.clone().into()),
            DataForm::Custom(interpreter) => {
                if let Some(interpreter) = interpreters.get(interpreter) {
                    interpreter.read(path, name, version)
                } else {
                    log_err!("Unknown interpreter: {interpreter}")
                }
            }
        }
    }

    pub fn write(
        &self,
        output: &Path,
        datasets: &DsmSets,
        interpreters: &HashMap<String, Interpreter>,
    ) -> anyhow::Result<()> {
        match self {
            DataForm::Yolo1_1 => yolo_1_1::write(output, datasets),
            DataForm::Coco1_0 => coco_1_0::write(output, datasets),
            DataForm::Custom(interpreter) => {
                if let Some(interpreter) = interpreters.get(interpreter) {
                    interpreter.write(output, datasets)
                } else {
                    log_err!("Unknown interpreter: {interpreter}")
                }
            }
        }
    }

    pub fn to_data_form(&self) -> DsmDataForm {
        match self {
            DataForm::Yolo1_1 => DsmDataForm::Yolo1_1,
            DataForm::Coco1_0 => DsmDataForm::Coco1_0,
            DataForm::Custom(custom) => DsmDataForm::Custom(custom.clone()),
        }
    }
}

impl From<DataForm> for DsmDataForm {
    fn from(form: DataForm) -> Self {
        match form {
            DataForm::Yolo1_1 => DsmDataForm::Yolo1_1,
            DataForm::Coco1_0 => DsmDataForm::Coco1_0,
            DataForm::Custom(f) => DsmDataForm::Custom(f.clone()),
        }
    }
}
