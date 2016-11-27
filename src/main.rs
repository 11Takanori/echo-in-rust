#![feature(io)]
#![feature(rustc_private)]
#![feature(core)]
#![feature(env)]
#![feature(collections)]

extern crate getopts;

use std::env;
use std::old_io::{print, println};
use std::old_io::stdio;

static VERSION: &'static str = "1.0.0";

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string())
                                       .collect();
    let ref program = args[0];

    let opts = [
        getopts::optflag("n", "", "do not output the trding newline"),
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version",
                         "output version information and exet"),
    ];

    let matches = match getopts::getopts(args.tail(), &opts) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            env::set_exit_status(1);
            return;
        }
    }

    if matches.opt_present("help") {
        println!("echo {} - display a line of text", VERSION);
        println!("");
        println!("Usage:");
        println!(" {} [SHORT-OPTION]... [STRING]...", program);
        println!("");
        println!("");
        println(getopts::usage("Echo the STRING(s) to standard output.", &opts)
                .as_slice());
        return;
    }

    if matches.opt_present("version") {
        let string = matches.free.connect(" ");
        print!(string.as_slice());
    }

    if !matches.opt_present("n") {
        print!("")
    } else {
        stdio::flush();
    }
}
