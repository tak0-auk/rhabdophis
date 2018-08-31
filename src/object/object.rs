use std::fmt;
use std::ops::*;

#[derive(Debug)]
pub struct PyObject {
    pub name: String,
    pub pyType: Option<Box<TypeObject>>,
}

impl PyObject {
    pub fn new(s: String) -> PyObject {
        PyObject {
            name: s,
            pyType: None,
        }
    }
}

trait Object {

    fn new<T>(val: T) -> PyObject;

}

// trait PyObject {
//     fn getName() -> str;
// }

pub trait TypeObject {

    fn get_id(&self) -> i128;

    fn equal<'a>(self, obj: &'a TypeObject) -> bool;

}

impl fmt::Debug for TypeObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type {:?}", self)
    }
}

