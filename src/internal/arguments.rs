use std::{env, process};
use std::path::PathBuf;

pub fn get_argument() -> PathBuf {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 || matches!(args.get(0).map(String::as_str), Some("--help" | "-h")) {
        print_usage();
        process::exit(0);
    }

    let folder: PathBuf = args.join(" ").into();

    if !folder.exists() {
        println!("provided file does not exist");
        process::exit(1);
    }

    folder
}

fn print_usage() {
    println!("DESCRIPTION: \tFinds duplicates in given folder and subfolders.");
    println!("\t\tUses HASH to categorise files");
    println!("USAGE: \t\t<program> STRING");
    println!("\t\tSTRING: path to the root folder");
}
