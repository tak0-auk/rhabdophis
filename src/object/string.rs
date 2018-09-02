#[derive(Debug)]
pub struct PyStringObject {
    value: String,
}

impl PyStringObject {

    pub fn new(s: String) -> PyStringObject {
        PyStringObject {
            value: s,
        }
    }
}