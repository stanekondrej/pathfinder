use std::{env, path, process};

fn read_dir(path: &path::Path) -> Vec<String> {
    if path.is_file() {
        return vec![path.to_string_lossy().to_string()];
    }

    let listing = path.read_dir();
    if listing.is_err() {
        return vec![];
    }
    listing
        .unwrap()
        .map(|d| read_dir(&d.expect("Failed to get file name").path()))
        .flatten()
        .collect()
}

fn main() {
    let path = std::env::var("PATH").expect("Your PATH is not set. You should probably set it lol");
    if path.len() == 0 {
        panic!("PATH is empty. wtf.")
    }
    let files = path
        .split(":")
        .map(|p| path::Path::new(p))
        .map(|d| read_dir(d))
        .flatten();

    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        eprintln!("No search terms provided.");
        process::exit(1);
    }

    files
        .filter(|x| args[1..].iter().fold(false, |b, arg| b || x.contains(arg)))
        .for_each(|r| println!("{r}"));
}
