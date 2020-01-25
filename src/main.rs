use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::{env, fs};
use walkdir::{DirEntry, WalkDir};

fn dir_iter<P: AsRef<Path>>(path: P) -> Vec<DirEntry> {
    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter(|x| x.as_ref().expect("Failed to process path").path().is_file())
        .map(|x| x.unwrap())
        .collect();

    return entries;
}

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

fn map_fs(paths: Vec<String>) -> HashMap<u64, Vec<PathBuf>> {
    let mut files: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for path in paths {
        for entry in dir_iter(path) {
            let key = fs::metadata(entry.path()).unwrap().len();
            let value = files.entry(key).or_insert(Vec::new());

            value.push(entry.path().to_path_buf());
        }
    }

    return files;
}

fn map_sd(files: HashMap<u64, Vec<PathBuf>>) -> HashMap<u64, Vec<PathBuf>> {
    let mut dupes: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for (key, v) in files {
        if v.len() == 1 {
            continue;
        }

        for p1 in &v {
            let mut f1 = File::open(p1).expect("Failed to open file");
            f1.seek(SeekFrom::Start(0))
                .expect("Failed to offset read to beginning");
            let mut b1: Vec<u8> = Vec::new();
            f1.read_to_end(&mut b1)
                .expect("Failed to read file to buffer");
            let mut dupe = false;

            for p2 in &v {
                if p1 == p2 {
                    continue;
                }

                let mut f2 = File::open(p2).expect("Failed to open file");
                f2.seek(SeekFrom::Start(0))
                    .expect("Failed to offset read to beginning");
                let mut b2: Vec<u8> = Vec::new();
                f2.read_to_end(&mut b2)
                    .expect("Failed to read file to buffer");

                if b1 != b2 {
                    continue;
                }

                let value = dupes.entry(key).or_insert(Vec::new());

                if !dupe {
                    dupe = true;

                    value.push(p1.to_path_buf());
                }

                value.push(p2.to_path_buf());
            }

            break;
        }
    }

    return dupes;
}

fn main() {
    let paths = input_paths();
    let files = map_fs(paths);
    let dupes = map_sd(files);
}
