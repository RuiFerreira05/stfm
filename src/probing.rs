use std::{
    env::{self, current_dir},
    fs::read_dir,
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        listdir(Path::new(&args[1]));
    } else {
        listdir(
            current_dir()
                .expect("could not read current directory")
                .as_path(),
        );
    }
}

fn listdir(path: &Path) {
    if let Ok(dirs) = read_dir(path) {
        for dir in dirs {
            if let Ok(dir) = dir {
                println!(
                    "{:?} - is dir: {:?}",
                    dir.file_name(),
                    dir.file_type().unwrap().is_dir()
                )
            }
        }
    } else {
        panic!("Could not read directory");
    }
}
