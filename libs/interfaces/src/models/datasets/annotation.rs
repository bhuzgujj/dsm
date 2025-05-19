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
	occluded: bool,
	rotation: u32
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
}

impl DsmAnnotation {
	pub fn new(
		class: u32,
		x: f64,
		y: f64,
		width: f64,
		height: f64,
		segmentation: Vec<f64>,
		iscrowd: u32,
		occluded: bool,
		rotation: u32
	) -> Self {
		Self {
			class,
			x,
			y,
			width,
			height,
			segmentation,
			iscrowd,
			occluded,
			rotation
		}
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

	pub fn get_class(&self) -> u32 {
		self.class
	}

	pub fn get_x(&self) -> f64 {
		self.x
	}

	pub fn get_y(&self) -> f64 {
		self.y
	}

	pub fn get_width(&self) -> f64 {
		self.width
	}

	pub fn get_height(&self) -> f64 {
		self.height
	}

	pub fn get_segmentation(&self) -> &Vec<f64> {
		&self.segmentation
	}

	pub fn get_area(&self) -> f64 {
		self.height * self.width
	}

	pub fn get_iscrowd(&self) -> u32 {
		self.iscrowd
	}

	pub fn get_occluded(&self) -> bool {
		self.occluded
	}
	
	pub fn get_rotation(&self) -> u32 {
		self.rotation
	}
}