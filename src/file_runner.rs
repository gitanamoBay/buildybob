extern crate postgres;

use std::io::prelude::*;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use self::postgres::{Connection, TlsMode};

use run_type;

pub fn run_file(path: &Path, db: &Connection, sql_t: run_type::SqlType) {
    let mut buffer = String::new();

    let name = path.file_stem().unwrap();



    if let Ok(mut file) = File::open(path) {
        file.read_to_string(&mut buffer);
    }

    if buffer.len() > 0 {
    //    db.execute(&buffer, &[]).unwrap();
    }
}