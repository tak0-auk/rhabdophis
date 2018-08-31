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

impl TypeObject for PyIntObject {

    fn get_id(&self) -> i128 {
        self.val
    }

    fn equal<'a>(self, obj: &'a TypeObject) -> bool {
        match obj {
            PyIntObject => self.get_id() == obj.get_id(),
            _ => panic!(""),
        }

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
#[test]
fn equal() {
    let p1 = PyIntObject::new(1);
    let p2 = PyIntObject::new(1);
    assert!(p1.equal(&p2));
}