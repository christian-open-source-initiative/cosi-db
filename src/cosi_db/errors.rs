// Error handling logic.
use std::error::Error;
use std::fmt::Display;

pub type COSIError = anyhow::Error;
pub type COSIResult<T> = Result<T, COSIError>;
