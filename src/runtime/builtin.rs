use std::io::{Write};

use object::object::PyObject;

pub fn print(obj: PyObject) {
    let s = obj.to_string();
    io::stdout().write(s.as_bytes());
}