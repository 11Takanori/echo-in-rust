#![feature(rustc_private)]

extern crate getopts;

use std::env;

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string())
                                       .collect();
    let ref program = args[0];

    let opts = [
        getopts::optflag("n", "", "do not output the trailing newline"),
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version",
                         "output version information and exet"),
    ];
}
