extern crate postgres;

use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::fs::{self, DirEntry};
use std::fs::File;
use std::path::Path;

use self::postgres::{Connection, TlsMode};

use file_runner;
use run_type;

pub fn run<P: AsRef<Path>>(root: P, db: &str) {
    let root = root.as_ref();

    let conn = Connection::connect(db, TlsMode::None).unwrap();

    let types = run_type::get_types();

    for sql_type in types {
        let path = root.join(sql_type.folder_name);

        if !path.is_dir() {
            panic!("{} could not be located", path.display());
        }

        traverse(&path, &conn, sql_type.sql_type.clone());
    }
}

fn traverse(path: &Path, db: &Connection, file_type: run_type::SqlType) {
    if let Ok(dir_entries) = fs::read_dir(path.clone()) {
        for dir_entry in dir_entries {
            if let Ok(dir_entry) = dir_entry {
                let entry_path = dir_entry.path();

                if entry_path.is_dir() {
                    traverse(&entry_path, &db, file_type.clone());
                } else {
                    if entry_path.extension().unwrap() == "sql" {
                        file_runner::run_file(&entry_path, &db, file_type.clone());
                    }
                }
             }
         }
    }
}


