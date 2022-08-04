// Error handling logic.
use std::error::Error;

pub type RuntimeError = Box<dyn Error + Send>;
pub type CosiResult<T> = Result<T, RuntimeError>;
