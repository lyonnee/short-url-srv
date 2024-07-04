use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub struct NotFoundUserErr();

impl Display for NotFoundUserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not found user")
    }
}

impl Error for NotFoundUserErr {}
