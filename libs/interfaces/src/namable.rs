pub trait Namable {
	fn name(&self) -> String;
	fn version(&self) -> String;
	fn type_name() -> &'static str;
	fn log_name(&self) -> String {
		format!(
			"{} named '{}' version '{}'",
			Self::type_name(),
			self.name(),
			self.version()
		)
	}
}
