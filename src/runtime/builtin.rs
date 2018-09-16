use std::io::{ self, Write};

use object::object::PyObject;

pub fn print(obj: PyObject) {
    let s = obj.name;
    io::stdout().write(s.as_bytes());
}