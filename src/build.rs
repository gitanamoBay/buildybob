use std::path::Path;

pub fn run(root: &Path) {
    let folders = vec!["schema"];

    for folder in folders {
        let path = root.join(folder);

        print!("{}", path.display());
    }
}