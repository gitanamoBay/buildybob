use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;


pub fn run<P: AsRef<Path>>(root: P) {
    let root = root.as_ref();

    let folders = vec!["filegroup","partition", "schema", "table"];

    for folder in folders {
        let path = root.join(folder);
        print!("Deploying {}s readinging from {}", folder, path.display());
        
        if !path.is_dir() {
            panic!("{} could not be located", path.display());
        }
                                           
        for file in try!(fs::read_dir(path)) {
            
        }
        
        print!("Successfully deployed {}s in {}", folder, path.display());
    }
}