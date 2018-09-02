use std::ops::*; // operators
use object::object::TypeObject;

#[derive(Debug)]
pub struct PyIntObject {
    value: i128,
}

impl PyIntObject {

    pub fn new(i: i128) -> PyIntObject {
        PyIntObject {
            value: i
        }
    }

    pub fn get_value(self) -> i128 {
        self.value
    }

}

impl PyIntObject {

    fn equal(self, obj: PyIntObject) -> bool {
        self.value == obj.value
    }

}

impl TypeObject for PyIntObject {

    // fn get_id(&self) -> i128 {
    //     self.value
    // }

    fn get_type(self) -> String {
        "int".to_string()
    }

    // fn equal<'a>(self, obj: &'a TypeObject) -> bool {
    //     match obj {
    //         PyIntObject => self.get_id() == obj.get_id(),
    //         _ => panic!(""),
    //     }

    // }
}

impl Add for PyIntObject {
    type Output = PyIntObject;

    fn add(self, other: PyIntObject) -> PyIntObject {
        PyIntObject { value: self.value + other.value }
    }
}


#[test]
fn add() {
    assert_eq!((PyIntObject::new(1) + PyIntObject::new(10)).get_value(), 11);
}
#[test]
fn equal() {
    let p1 = PyIntObject::new(1);
    let p2 = PyIntObject::new(1);
    assert!(p1.equal(p2));
}