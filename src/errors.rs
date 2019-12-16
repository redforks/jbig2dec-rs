use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),

    CreateContextFailed,
    IncompletePage
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}