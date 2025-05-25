use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct UiError {
	error: String,
}

impl From<anyhow::Error> for UiError {
	fn from(value: anyhow::Error) -> Self {
		Self {
			error: format!("{}", value),
		}
	}
}

impl From<std::io::Error> for UiError {
	fn from(value: std::io::Error) -> Self {
		Self {
			error: format!("{}", value),
		}
	}
}

impl Display for UiError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.error)
	}
}

impl Error for UiError {
	
}