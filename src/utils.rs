use std::{fs::File, io::prelude::*, path::PathBuf};

pub fn read_to_string<P: Into<PathBuf>>(path: P) -> String {
    let mut file = File::open(path.into()).unwrap();
    let mut input_txt = String::new();
    file.read_to_string(&mut input_txt).unwrap();
    input_txt
}
