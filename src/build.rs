use std::path::Path;

pub fn run<P: AsRef<Path>>(root: P) {
    let root = root.as_ref();

    let folders = vec!["filegroup","partition", "schema", "table"];

    for folder in folders {
        let path = root.join(folder);
        print!("Deploying {}s readinging from {}", folder, path);
    
        
        print!("Successfully deployed {}s in {}", folder, path);
    }
}