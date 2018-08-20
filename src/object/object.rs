use std::fmt;

#[derive(Debug)]
pub struct PyObject {
    pub pyType: TypeObject,
}

trait Object {

}


pub trait TypeObject {

    fn equal(self, obj: TypeObject) -> bool;

}

impl fmt::Debug for TypeObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type {:?}", self)
    }
}

