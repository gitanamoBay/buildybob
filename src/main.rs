extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

mod build;

const USAGE: &'static str = "
buildybob deploys database.

Usage:
    buildybob <path> [--clean | --server=<str>]
    
Options:
    -h --help       Show this screen.
    --server=<str>   Server
    --version       Show Version.
    --clean         Deploy fresh copy of database.
";

const DEFAULT_SERVER: &'static str = "postgresql://postgres@localhost";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_clean: bool,
    flag_server: String,
    arg_path: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    let mut server: String = DEFAULT_SERVER.to_string();

    if !args.flag_server.is_empty() {
        server = args.flag_server.to_string();
    }

    build::run(args.arg_path, &server);
}
