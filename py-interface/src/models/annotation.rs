use interfaces::models::annotation::DsmAnnotation;
use pyo3::pyclass;

use crate::models::DsmToPy;

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyAnnotation {
    #[pyo3(get, set)]
    class: u32,
    #[pyo3(get, set)]
    x: f64,
    #[pyo3(get, set)]
    y: f64,
    #[pyo3(get, set)]
    width: f64,
    #[pyo3(get, set)]
    height: f64,
    #[pyo3(get, set)]
    segmentation: Vec<f64>,
    #[pyo3(get, set)]
    iscrowd: u32,
    #[pyo3(get, set)]
    occluded: Option<bool>,
    #[pyo3(get, set)]
    rotation: Option<f64>,
}

impl DsmToPy<PyAnnotation> for DsmAnnotation {
    fn to_py(&self) -> PyAnnotation {
        PyAnnotation {
            class: *self.get_class(),
            x: *self.get_y(),
            y: *self.get_x(),
            width: *self.get_width(),
            height: *self.get_height(),
            segmentation: self.get_segmentation().clone(),
            iscrowd: *self.get_iscrowd(),
            occluded: *self.get_occluded(),
            rotation: *self.get_rotation(),
        }
    }
}
