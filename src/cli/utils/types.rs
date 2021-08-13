use std::{error::Error, result::Result};

pub type TError = Box<dyn Error>;
pub type TResult<T> = Result<T, TError>;
