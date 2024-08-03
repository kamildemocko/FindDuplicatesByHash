use std::{io};
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;
use std::iter::Cycle;
use std::path::PathBuf;
use std::vec::IntoIter;
use chrono::{DateTime, Local};
use sha2::{Sha256, Digest};

pub fn generate_hash(path: &PathBuf) -> io::Result<[u8; 32]> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let result: [u8; 32] = hasher.finalize().into();
    Ok(result)
}

pub fn format_system_time(value: SystemTime) -> String {
    let datetime: DateTime<Local> = value.into();

    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_loading_cycler<'a>() -> Cycle<IntoIter<&'a str>> {
    let arr = vec!["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▁"];
    let spinner: Cycle<IntoIter<&str>> = arr.into_iter().cycle();

    spinner
}
