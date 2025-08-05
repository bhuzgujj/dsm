use interfaces::models::annotation::DsmAnnotation;
use pyo3::pyclass;
use pyo3_stub_gen_derive::gen_stub_pyclass;
use crate::models::DsmToPy;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyAnnotation {
    #[pyo3(get, set)]
    class_id: u32,
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
            class_id: *self.class(),
            x: *self.y(),
            y: *self.x(),
            width: *self.width(),
            height: *self.height(),
            segmentation: self.segmentation().clone(),
            iscrowd: *self.iscrowd(),
            occluded: *self.occluded(),
            rotation: *self.rotation(),
        }
    }
}
