use std::error;
use std::fmt;

#[derive(Debug)]
enum PythonError {
    Indentation,
}

impl fmt::Display for PythonError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PythonError::Indentation => write!(f, "{}", ""),
        }
    }
}

// IndentationError: unindent does not match any outer indentation level