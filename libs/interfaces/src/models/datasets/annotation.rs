#[derive(Debug, Clone)]
pub struct Annotation {
	class: u32,
	x: f64,
	y: f64,
	width: f64,
	height: f64,
}

impl Annotation {
	pub fn to_file_str(&self) -> String {
		format!("{} {:.6} {:.6} {:.6} {:.6}", self.class, self.x, self.y, self.width, self.height)
	}
}

impl Annotation {
	pub fn new(class: u32, x: f64, y: f64, width: f64, height: f64) -> Self {
		Self {
			class,
			x,
			y,
			width,
			height,
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
}