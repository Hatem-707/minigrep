use std::{
    env,
    fs::{self, File},
    io::{ErrorKind, Read, stdin},
    process::exit,
};

use minigrep::minigrep::{SearchInterface, Searchable};

fn run<T>(pat: &str, ignore_case: Option<bool>, input_src: T)
where
    T: Read,
{
    let mut app = SearchInterface::new(ignore_case, input_src);
    let lines = app.search(&pat).expect("couldn't search for pattern");
    for line in lines {
        println!("{}", line.trim());
    }
}

fn process_path(path: &str) -> Option<File> {
    let file = match fs::File::open(path) {
        Ok(f) => Some(f),
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => eprintln!("Couldn't find file: {}", path),

                _ => eprintln!("Error: {}", e.to_string()),
            };
            None
        }
    };
    file
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: minigrep [String] [File Path] or pipe input to the program");
        std::process::exit(1);
    }
    let mut ignore_case: Option<bool> = None;
    let mut index: usize = 2;
    let pat = if args[1] == "-i" {
        ignore_case = Some(true);
        index += 1;
        &args[2]
    } else {
        &args[1]
    };
    let file_path = args.get(index);
    if let Some(path) = file_path {
        let Some(file) = process_path(path) else {
            exit(1);
        };
        run(pat, ignore_case, file);
    } else {
        run(pat, ignore_case, stdin());
    }
}
