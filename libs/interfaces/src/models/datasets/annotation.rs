#[derive(Debug, Clone)]
pub struct Annotation {
	class: String,
	x: f64,
	y: f64,
	width: f64,
	height: f64,
}

impl Annotation {
	pub fn new(class: &String, x: f64, y: f64, width: f64, height: f64) -> Self {
		Self {
			class: class.clone(),
			x,
			y,
			width,
			height,
		}
	}

	pub fn get_class(&self) -> String {
		self.class.clone()
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
}