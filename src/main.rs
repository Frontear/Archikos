use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use walkdir::{DirEntry, WalkDir};

fn input_paths() -> Vec<String> {
    let mut paths: Vec<_> = env::args()
        .skip(1) // ignore binary path
        .filter(|x| {
            let path = Path::new(&x);

            if !path.is_dir() {
                eprintln!("{} is not a valid directory (skipping)", path.display());
                return false;
            }

            return true;
        })
        .collect();

    paths.sort();
    paths.dedup(); // remove duplicate paths

    return paths;
}

fn dir_iter<P: AsRef<Path>>(path: P) -> Vec<DirEntry> {
    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter(|x| x.as_ref().expect("Failed to process path").path().is_file())
        .map(|x| x.unwrap())
        .collect();

    return entries;
}

fn main() {
    let paths = input_paths();

    let mut files: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for path in paths {
        for entry in dir_iter(path) {
            let key = fs::metadata(entry.path()).unwrap().len();
            let value = files.entry(key).or_insert(Vec::new());

            value.push(entry.path().to_path_buf());
        }
    }

    println!("{:#?}", files);
}
