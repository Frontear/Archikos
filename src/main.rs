use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect(); // path argument not necessary

    for arg in &args {
        let path = Path::new(&arg);

        if !path.is_dir() {
            eprintln!("{} is not a valid directory (skipping)", path.display());
        }
    }
}
