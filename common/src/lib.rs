use std::{
    fs,
    io::{
        self,
        Read
    },
    path::Path,
};

// Options to pass to the solver for each day
pub struct Options {
    pub debug: bool,
    pub iterations: Option<u32>,
}
impl Default for Options {
    fn default() -> Self {
        Options { debug: false, iterations: None }
    }
}

// Read puzzle input from a file, or from stdin if the path is "-".
pub fn read_input(path: &Path) -> io::Result<String> {
    if path == Path::new("-") {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s)?;
        Ok(s)
    } else {
        fs::read_to_string(path)
    }
}