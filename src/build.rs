extern crate postgres;

use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::fs::{self, DirEntry};
use std::fs::File;
use std::path::Path;

use self::postgres::{Connection, TlsMode};

pub fn run<P: AsRef<Path>>(root: P, db: &str) {
    let root = root.as_ref();

    let conn = Connection::connect(db, TlsMode::None).unwrap();

    let folders = vec!["filegroup", "partition", "schema", "table"];

    for folder in folders {
        let path = root.join(folder);

        if !path.is_dir() {
            panic!("{} could not be located", path.display());
        }

        if let Ok(dir_entries) = fs::read_dir(path.clone()) {
            for dir_entry in dir_entries {
                if let Ok(dir_entry) = dir_entry {
                    run_file(&dir_entry.path(), &conn);
                }
            }
        }
    }
}

fn traverse() {
    
}

fn run_file(path: &Path, db: &Connection) {
    let mut buffer = String::new();

    if let Ok(mut file) = File::open(path) {
        file.read_to_string(&mut buffer);
    }

    db.execute(&buffer, &[]).unwrap();
}
