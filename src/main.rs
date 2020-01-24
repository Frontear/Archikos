use std::env;
use std::path::Path;

fn main() {
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
    println!("{:?}", paths);
}
