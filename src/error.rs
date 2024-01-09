use std::{fmt::Debug, result};

/// This enum contains variants of errors that could be returned by the API methods
/// that this library provides.
#[derive(PartialEq)]
pub enum Error {}

impl Debug for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type Result<T> = result::Result<T, Error>;
