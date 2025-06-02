use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DsmAnnotation {
	class: u32,
	x: f64,
	y: f64,
	width: f64,
	height: f64,
	segmentation: Vec<f64>,
	iscrowd: u32,
	occluded: Option<bool>,
	rotation: Option<f64>
}

impl DsmAnnotation {
	pub fn to_file_str(&self) -> String {
		format!("{} {:.6} {:.6} {:.6} {:.6}", self.class, self.x, self.y, self.width, self.height)
	}

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
			rotation: self.rotation
		}
	}

	pub fn get_class(&self) -> &u32 {
		&self.class
	}

	pub fn get_x(&self) -> &f64 {
		&self.x
	}

	pub fn get_y(&self) -> &f64 {
		&self.y
	}

	pub fn get_width(&self) -> &f64 {
		&self.width
	}

	pub fn get_height(&self) -> &f64 {
		&self.height
	}

	pub fn get_segmentation(&self) -> &Vec<f64> {
		&self.segmentation
	}

	pub fn get_area(&self) -> f64 {
		self.height * self.width
	}

	pub fn get_iscrowd(&self) -> &u32 {
		&self.iscrowd
	}

	pub fn get_occluded(&self) -> &Option<bool> {
		&self.occluded
	}
	
	pub fn get_rotation(&self) -> &Option<f64> {
		&self.rotation
	}
}

pub struct DsmAnnotationBuilder {
	class: u32,
	x: f64,
	y: f64,
	width: f64,
	height: f64,
	segmentation: Vec<f64>,
	iscrowd: u32,
	occluded: Option<bool>,
	rotation: Option<f64>
}

impl DsmAnnotationBuilder {
	pub fn builder(
		class: u32,
		x: f64,
		y: f64,
		width: f64,
		height: f64
	) -> Self {
		Self {
			class,
			x,
			y,
			width,
			height,
			segmentation: Vec::new(),
			iscrowd: 0,
			occluded: None,
			rotation: None
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

	pub fn build(self) -> DsmAnnotation {
		DsmAnnotation {
			class: self.class,
			x: self.x,
			y: self.y,
			width: self.width,
			height: self.height,
			segmentation: self.segmentation,
			iscrowd: self.iscrowd,
			occluded: self.occluded,
			rotation: self.rotation
		}
	}
}