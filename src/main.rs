use std::{
    env::{self, current_dir},
    path::{Path, PathBuf},
};
use stfm::{app::App, errors::AppError};

fn main() -> Result<(), AppError> {
    // Extracting arg path
    let args: Vec<String> = env::args().collect();
    let path: PathBuf;
    if args.len() > 1 {
        path = Path::new(&args[1]).to_path_buf();
    } else {
        let cur_dir = current_dir().unwrap();
        path = cur_dir;
    }

    // Creating new app instance
    let mut app = App::new(path)?;

    // Running update loop
    ratatui::run(|terminal| app.run(terminal)).expect("Oops, something went wrong");

    Ok(())
}
