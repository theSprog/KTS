use std::fs::{self, File};
use std::io::{BufReader, Read};

use crate::error::err_exit;

pub(crate) fn get_char_stream(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Ok(file) => BufReader::new(file),
        Err(err) => panic!("err: {}", err),
    };
    let file_size = fs::metadata(filename).unwrap().len() + 1024;
    let mut char_stream = String::with_capacity(file_size.try_into().unwrap());
    if let Err(err) = file.read_to_string(&mut char_stream) {
        err_exit(err);
    }
    char_stream
}
