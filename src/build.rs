use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;


pub fn run<P: AsRef<Path>>(root: P, db: &str) {
    let root = root.as_ref();

    println!("{}", db);

    let folders = vec!["filegroup","partition", "schema", "table"];

    for folder in folders {
        let path = root.join(folder);
        
        println!("Deploying {}s readinging from {}", folder, path.display());
        
        if !path.is_dir() {
            panic!("{} could not be located", path.display());
        }

        let files =  fs::read_dir(path.clone()).unwrap();
                                
        for file in files {
            
        }
        
        println!("Successfully deployed {}s in {}", folder, path.display());
    }
}