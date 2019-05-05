extern crate getopts;

use getopts::Options;
use std::env;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::process;

fn usage(opts: Options) -> String {
    opts.usage("Usage: soak [options] FILE")
}

fn pipe(reader: &mut Read, writer: &mut Write) {
    let mut buffer = [0u8; 8 * 1024];
    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        writer.write_all(&buffer[..n]).unwrap();
    }
}

fn open_file(path: &str, append: bool) -> File {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(!append)
        .append(append)
        .open(path)
        .unwrap()
}

fn main() {
    let mut opts = Options::new();
    opts.optflag("a", "", "append to file instead of overwriting");
    opts.optflag("h", "help", "print this help menu");

    let args = env::args_os().skip(1);
    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(_) => {
            eprintln!("{}", usage(opts));
            process::exit(1);
        }
    };
    if matches.opt_present("h") {
        return println!("{}", usage(opts));
    }

    let mut out: Box<Write> = match matches.free.first() {
        None => Box::new(io::stdout()),
        Some(file) => {
            let append = matches.opt_present("a");
            Box::new(open_file(file, append))
        }
    };
    pipe(&mut io::stdin(), &mut out);
}
