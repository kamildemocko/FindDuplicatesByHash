use std::{fs, io};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use crate::internal::utils;

pub struct App<'a> {
    root_folder: &'a PathBuf,
    db: HashMap<[u8; 32], Vec<PathBuf>>,
}

impl App<'_> {
    pub fn new(root_folder: &PathBuf) -> App {
        App{ root_folder, db: HashMap::new() }
    }

    pub fn run(&mut self) {
        let mut current_file_c: usize = 0;
        let files_count = utils::count_files_with_glob(self.root_folder);

        self.load_db(self.root_folder, files_count, &mut current_file_c);
        println!();

        let duplicates = self.get_duplicates_from_db();
        self.print_duplicates(duplicates);
    }

    fn load_db(&mut self, folder: &PathBuf, files_count: usize, current_file_c: &mut usize) {
        let mut spinner = utils::get_loading_cycler();

        let folder_glob: glob::Paths =
            glob::glob(&format!("{}/{}", folder.to_str().unwrap(), "*"))
                .expect("glob func failed");

        for file in folder_glob {
            let file_path = file.unwrap();

            match file_path.is_dir() {
                true => {
                    self.load_db(&file_path, files_count, current_file_c);
                },
                false => {
                    let gen_hash = utils::generate_hash(&file_path).unwrap();
                    self.insert_file_hash(gen_hash, &file_path);
                    *current_file_c += 1;
                },
            }

            print!("\r{} (File {} of {})", spinner.next().unwrap(), current_file_c, files_count);
            io::stdout().flush().unwrap()
        }
    }

    fn insert_file_hash(&mut self, gen_hash: [u8; 32], file: &PathBuf) {
        self.db.entry(gen_hash)
            .and_modify(|vec| vec.push(file.clone()))
            .or_insert(vec![file.clone()]);
    }

    fn get_duplicates_from_db(&self) -> Vec<[u8; 32]> {
        let mut duplicates: Vec<[u8; 32]> = vec![];

        self.db
            .iter()
            .for_each(|(key, value)| {
                if value.len() > 1 { duplicates.push(*key) }
            });

        duplicates
    }

    fn print_duplicates(&self, duplicates: Vec<[u8; 32]>) {
        if duplicates.len() == 0 {
            println!("No duplicate found");
            return
        }

        println!("Found duplicates:");
        println!("{}", "-".repeat(80));

        for duplicate in duplicates {
            let values = self.db.get(&duplicate).unwrap();

            for value in values {
                println!("{}", value.to_str().unwrap())
            }

            let first = values.get(0).unwrap();
            let metadata = fs::metadata(first).unwrap();
            let file_size_bytes = metadata.len();
            let created = utils::format_system_time(metadata.created().unwrap());

            println!("[size: {} MB | created: {}]",
                     file_size_bytes / 1024 / 1024,
                     created,
            );

            println!("{}", "-".repeat(80));
        }
    }
}
