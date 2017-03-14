use std::path::Path;

pub fn run<P: AsRef<Path>>(root: P) {
    let root = root.as_ref();

    let folders = vec!["schema"];

    for folder in folders {
        let path = root.join(folder);

        print!("{}", path.display());
    }
}