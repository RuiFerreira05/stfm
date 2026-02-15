use std::io::Error;

#[derive(Debug)]
pub enum AppError {
    GenericAppError(Error),
    DirReadError(Error),
    GhostPath,
    ReadEventErr(Error),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::GenericAppError(value)
    }
}
