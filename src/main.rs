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
        let cur_dir = current_dir().map_err(|e| AppError::NoCwd(e))?;
        path = cur_dir;
    }

    // Creating new app instance
    let mut app = App::new(path)?;
    app.logger
        .log_info("App startup: instance created, entering run loop");

    // Running update loop
    if let Err(e) = ratatui::run(|terminal| app.run(terminal)) {
        app.logger
            .log_error(format!("App crashed with error: {}", e).as_str());
        println!(
            "The app seems to have crashed with the following error: {}",
            e
        )
    }

    #[cfg(debug_assertions)]
    println!("------------------\n{:?}\n-------------------", app.logger);
    print!("{}", app.output);
    Ok(())
}
