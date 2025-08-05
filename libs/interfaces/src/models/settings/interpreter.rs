use std::{path::Path, process::Command};

use serde::{Deserialize, Serialize};

use crate::{log_err, models::DsmSets};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Interpreter {
    process: String,
    args: Vec<String>,
}

impl Interpreter {
    pub fn read(
        &self,
        path: &std::path::Path,
        name: Option<String>,
        version: String,
    ) -> anyhow::Result<Vec<DsmSets>> {
        let new_name = name.clone().unwrap_or(
            path.file_name()
                .expect("Could not get the directory name")
                .to_string_lossy()
                .into(),
        );
        let result = Command::new(&self.process)
            .args(&self.args)
            .arg("read")
            .arg(path)
            .arg(new_name)
            .arg(version)
            .output()?;

        let output = String::from_utf8(result.stdout)?;
        match serde_json::from_str::<Vec<DsmSets>>(&output) {
            Ok(sets) => Ok(sets),
            Err(_) => log_err!(format!("Error from the custom interpreter: {output}")),
        }
    }

    pub fn write(&self, input: &Path, output: &Path, datasets: &DsmSets) -> anyhow::Result<()> {
        let result = Command::new(&self.process)
            .args(&self.args)
            .arg("write")
            .arg(output)
            .arg(input)
            .output()?;

        let output = String::from_utf8(result.stdout)?;
        if output.len() > 0 {
            return log_err!(format!("Error from the custom interpreter: {output}"));
        }
        Ok(())
    }
}
