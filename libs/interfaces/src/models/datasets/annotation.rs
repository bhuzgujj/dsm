use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Annotation {
	class: u32,
	x: f64,
	y: f64,
	width: f64,
	height: f64,
	segmentation: Vec<f64>,
	iscrowd: u32,
	occluded: Option<bool>,
	rotation: Option<f64>,
}

impl Annotation {
	pub fn to_file_percent_str(&self, width: u32, height: u32) -> String {
		format!(
			"{} {:.6} {:.6} {:.6} {:.6}",
			self.class,
			self.x / width as f64,
			self.y / height as f64,
			self.width / width as f64,
			self.height / height as f64
		)
	}

	pub fn map_in(&self, new_class: u32) -> Self {
		Self {
			class: new_class,
			x: self.x,
			y: self.y,
			width: self.width,
			height: self.height,
			segmentation: self.segmentation.clone(),
			iscrowd: self.iscrowd,
			occluded: self.occluded,
			rotation: self.rotation,
		}
	}

	pub fn class(&self) -> &u32 {
		&self.class
	}

	pub fn x(&self) -> &f64 {
		&self.x
	}

	pub fn y(&self) -> &f64 {
		&self.y
	}

	pub fn width(&self) -> &f64 {
		&self.width
	}

	pub fn height(&self) -> &f64 {
		&self.height
	}

	pub fn segmentation(&self) -> &Vec<f64> {
		&self.segmentation
	}

	pub fn area(&self) -> f64 {
		self.height * self.width
	}

	pub fn iscrowd(&self) -> &u32 {
		&self.iscrowd
	}

	pub fn occluded(&self) -> &Option<bool> {
		&self.occluded
	}

	pub fn rotation(&self) -> &Option<f64> {
		&self.rotation
	}
}

pub struct AnnotationBuilder {
	class: u32,
	x: f64,
	y: f64,
	width: f64,
	height: f64,
	segmentation: Vec<f64>,
	iscrowd: u32,
	occluded: Option<bool>,
	rotation: Option<f64>,
}

impl AnnotationBuilder {
	pub fn builder(class: u32, x: f64, y: f64, width: f64, height: f64) -> Self {
		Self {
			class,
			x,
			y,
			width,
			height,
			segmentation: Vec::new(),
			iscrowd: 0,
			occluded: None,
			rotation: None,
		}
	}

	pub fn set_segmentation(mut self, segmentation: Vec<f64>) -> Self {
		self.segmentation = segmentation;
		self
	}

	pub fn set_iscrowd(mut self, iscrowd: u32) -> Self {
		self.iscrowd = iscrowd;
		self
	}

	pub fn set_occluded(mut self, occluded: Option<bool>) -> Self {
		self.occluded = occluded;
		self
	}

	pub fn set_rotation(mut self, rotation: Option<f64>) -> Self {
		self.rotation = rotation;
		self
	}

	pub fn build(self) -> Annotation {
		Annotation {
			class: self.class,
			x: self.x,
			y: self.y,
			width: self.width,
			height: self.height,
			segmentation: self.segmentation,
			iscrowd: self.iscrowd,
			occluded: self.occluded,
			rotation: self.rotation,
		}
	}
}
