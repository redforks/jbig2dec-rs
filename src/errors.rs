use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),

    CreateContextFailed(String),
    IncompletePage(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::CreateContextFailed(ref e) => write!(f, "create context failed: {}", e),
            Error::IncompletePage(ref e) => write!(f, "incomplete page: {}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Io(ref e) => Some(e),
            _ => None,
        }
    }
}
