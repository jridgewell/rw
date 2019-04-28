extern crate getopts;
extern crate tempfile;

use getopts::Options;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::SeekFrom;
use std::string::String;
use tempfile::tempfile;

fn print_usage() {
    print!("Usage: soak [options] FILE");
    print!("");
    print!("Options:");
    print!("-a            append to file instead of overwriting");
    print!("-h, --help    print this help menu");
}

fn pipe(reader: &mut Read, writer: &mut Write) {
    let mut data = [0u8; 1024 * 8];
    while let Ok(n) = reader.read(&mut data) {
        if n == 0 {
            break;
        }
        writer.write_all(&data[..n]).unwrap();
    }
}

fn open_file(path: &str, append: bool) -> File {
    return OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(!append)
        .append(append)
        .open(path)
        .unwrap();
}

fn outfile(path: &str, append: bool) -> Box<Write> {
    if path.is_empty() {
        return Box::new(std::io::stdout());
    }
    return Box::new(open_file(path, append));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("a", "", "append to file instead of overwriting");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("h") {
        return print_usage();
    }

    let append = matches.opt_present("a");
    let file = match matches.free.first() {
        None => "",
        Some(file) => file,
    };

    let mut tmp = tempfile().unwrap();
    pipe(&mut std::io::stdin(), &mut tmp);
    tmp.seek(SeekFrom::Start(0)).unwrap();
    pipe(&mut tmp, &mut outfile(file, append));
}
