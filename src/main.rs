#![feature(path_ext, plugin)]
#![plugin(docopt_macros)]

extern crate docopt;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use docopt::Docopt;
use rustc_serialize::json;

docopt!(Args derive Debug, "
    buildybob builds deploys database.

Usage:
    buildybob clean <path>
");

fn main() {
    let args: Args = Argss::docopt().decode().unwrap_or_else(|e| e.exit());



}