use clap::Parser;
use reqwest::Client;
use std::env;

// Algo:
// look for option. set option
// create a client.
// Make a request on the URL

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// URL from wich download occurs
    path: String,

    /// OP
    r: bool,
    l: Option<u32>,
    p: Option<String>,
}

impl Param {
    fn new(args: Vec<String>) -> Param {
        for arg in args {}
    }
}

fn check_for_option(args: Vec<String>, option: &mut (bool, u32, str)) {
    for arg in args {
        if arg == "-r" {
            option.0 = true;
        }
        if arg == "-l" {
            option.1 = arg + 1;
        }
        if arg == "-p" {
            option.2 = true;
        }
    }
}

fn main() {
    let param: Param = Param::new(env::args().collect());
    let client = Client::new();
}
