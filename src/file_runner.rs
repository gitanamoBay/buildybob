extern crate postgres;

use std::io::Read;
use std::fs::File;
use std::path::Path;

use self::postgres::Connection;

use run_type;

pub fn run_file(path: &Path, db: &Connection, sql_t: run_type::SqlType) {
    let mut buffer = String::new();

    let name = path.file_stem().unwrap();


    if let Ok(mut file) = File::open(path) {
        if let Ok(read_length) = file.read_to_string(&mut buffer) {
            if read_length <= 0 {
                return;
            }
        }
    }
    //    db.execute(&buffer, &[]).unwrap();


}
