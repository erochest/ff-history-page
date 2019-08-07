use std::env;
use std::error;
use std::fmt;
use std::convert::From;
use std::result;

use diesel;
use dotenv;

#[derive(Debug)]
pub enum HistoryError {
    DotEnvError(dotenv::Error),
    MissingEnvVar(env::VarError),
    ConnectionError(diesel::ConnectionError),
    ResultError(diesel::result::Error),
}

pub type Result<R> = result::Result<R, HistoryError>;

use HistoryError::*;

impl fmt::Display for HistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DotEnvError(ref err) => err.fmt(f),
            MissingEnvVar(ref err) => err.fmt(f),
            ConnectionError(ref err) => err.fmt(f),
            ResultError(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for HistoryError {
    fn description(&self) -> &str {
        match self {
            DotEnvError(_) => "dotenv error",
            MissingEnvVar(ref err) => err.description(),
            ConnectionError(ref err) => err.description(),
            ResultError(ref err) => err.description(),
        }
    }
}

impl From<dotenv::Error> for HistoryError {
    fn from(err: dotenv::Error) -> HistoryError {
        DotEnvError(err)
    }
}

impl From<env::VarError> for HistoryError {
    fn from(err: env::VarError) -> HistoryError {
        MissingEnvVar(err)
    }
}

impl From<diesel::ConnectionError> for HistoryError {
    fn from(err: diesel::ConnectionError) -> HistoryError {
        ConnectionError(err)
    }
}

impl From<diesel::result::Error> for HistoryError {
    fn from(err: diesel::result::Error) -> HistoryError {
        ResultError(err)
    }
}
