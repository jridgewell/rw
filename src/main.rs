extern crate getopts;
extern crate tempfile;

use getopts::Options;
use std::env;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use tempfile::tempfile;

fn print_usage() {
    println!("Usage: soak [options] FILE");
    println!("");
    println!("Options:");
    println!("  -a            append to file instead of overwriting");
    println!("  -h, --help    print this help menu");
}

fn pipe(reader: &mut Read, writer: &mut Write, buf: &mut [u8]) {
    while let Ok(n) = reader.read(buf) {
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n]).unwrap();
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
    let matches = opts.parse(args).unwrap();
    if matches.opt_present("h") {
        return print_usage();
    }

    let mut tmp = tempfile().unwrap();
    let mut buffer = [0u8; 8 * 1024];
    pipe(&mut io::stdin(), &mut tmp, &mut buffer);

    tmp.seek(SeekFrom::Start(0)).unwrap();
    let mut out: Box<Write> = match matches.free.first() {
        None => Box::new(io::stdout()),
        Some(file) => {
            let append = matches.opt_present("a");
            Box::new(open_file(file, append))
        }
    };
    pipe(&mut tmp, &mut out, &mut buffer);
}
