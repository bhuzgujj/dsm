use std::path::PathBuf;

const DSM_DIR: &'static str = ".dsm";

pub fn dsm_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory, wtf?")
        .join(DSM_DIR)
}
