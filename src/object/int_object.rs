use std::ops::{ Add };
use object::object::TypeObject;

#[derive(Debug)]
pub struct PyIntObject {
    val: i128,
}

impl PyIntObject {

    pub fn new(i: i128) -> PyIntObject {
        PyIntObject {
            val: i
        }
    }

    pub fn eval(self) -> i128 {
        self.val
    }
}

// TODO: commonize
impl PyIntObject {

    pub fn equal(self, obj: PyIntObject) -> bool {
        self.val == obj.val
    }

}

impl Add for PyIntObject {
    type Output = PyIntObject;

    fn add(self, other: PyIntObject) -> PyIntObject {
        PyIntObject { val: self.val + other.val }
    }
}


#[test]
fn add() {
    assert_eq!((PyIntObject::new(1) + PyIntObject::new(10)).eval(), 11);
}