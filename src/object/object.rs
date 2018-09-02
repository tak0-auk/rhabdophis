use std::fmt;
use std::ops::*;

#[derive(Debug)]
pub struct PyObject {
    pub name: String,
    pub py_type: Option<Box<TypeObject>>,
}

impl PyObject {
    pub fn new(s: String) -> PyObject {
        PyObject {
            name: s,
            py_type: None,
        }
    }

    // pub fn set_type(self, pt: &mut  TypeObject) {
    //     self.py_type = Some(Box::new(*pt));
    // }
}

trait Object {

    fn new<T>(val: T) -> PyObject;

}

// trait PyObject {
//     fn getName() -> str;
// }

pub trait TypeObject {

    // fn get_id(&self) -> i128;

    fn get_type(self) -> String;

    // fn equal<'a>(self, obj: &'a TypeObject) -> bool;

}

impl fmt::Debug for TypeObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type {:?}", self)
    }
}

