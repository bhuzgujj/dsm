use std::fs;
use std::path::Path;

pub fn copy_recursively(src: &Path, dst: &Path) -> anyhow::Result<()> {
	fs::create_dir_all(dst)?;
	for entry in fs::read_dir(src)? {
		let entry = entry?;
		let ty = entry.file_type()?;
		if ty.is_dir() {
			copy_recursively(&entry.path(), &dst.join(entry.file_name()))?;
		} else {
			fs::copy(entry.path(), dst.join(entry.file_name()))?;
		}
	}
	Ok(())
}