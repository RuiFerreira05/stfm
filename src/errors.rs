use std::io::Error;

use strum::Display;

#[derive(Debug, Display)]
pub enum AppError {
    DirReadError(Error),
    GhostPath,
    ReadEventErr(Error),
    NoCwd(Error),
}
