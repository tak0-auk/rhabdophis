use parser::token::Token;

use object::object::PyObject;

#[derive(Debug)]
pub struct Node {
    leaf: Token,

    // left: Option<Box<PyObject>>,
    // right: Box<PyObject>,
}