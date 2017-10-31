extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

mod build;
mod file_runner;
mod run_type;

const USAGE: &'static str = "
buildybob deploys database.

Usage:
    buildybob <path> [--clean | --configfile=<str> | --export=<str>]
    buildybob <configfile>
    
Options:
    -h --help               Show this screen.
    --version               Show Version.
    --clean                 Deploy fresh copy of database.
    --export=<str>          Exports config to name
    --configfile=<str>      Which meta file to use at root      
";

const DEFAULT_SERVER: &'static str = "postgresql://postgres@localhost";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_clean: bool,
    arg_configfile: String,
    arg_path: String,
}


struct ConfigFile {
    server: String,
    db_name: String,
    clean: bool,
    port: i32
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    let mut server: String = DEFAULT_SERVER.to_string();

    if !args.flag_server.is_empty() {
        server = args.flag_server.to_string();
    }
    
    if !args.flag_configfile.is_empty() {

    }

    build::run(args.arg_path, &server);
}
