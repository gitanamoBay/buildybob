extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

mod build;

const USAGE: &'static str = "
    buildybob deploys database.

Usage:
    buildybob <path> [--clean]
Options:
    -h --help       Show this screen.
    --version       Show Version.
    --clean         Deploy fresh copy of database.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_clean: bool,
    arg_path: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    
    let path = std::path::Path::new(&args.arg_path);

    build::run(&path);
}