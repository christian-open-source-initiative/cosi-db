// Error handling logic.
pub type COSIError = anyhow::Error;
pub type COSIResult<T> = Result<T, COSIError>;
