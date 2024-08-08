use std::{num::TryFromIntError, result};

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("TryFromIntError")]
    TryFromIntError(#[from] TryFromIntError),
    #[error("Failed to create reader")]
    FailedToCreate,
    #[error("Failed to read cpu instruction number")]
    FailedToRead,
    #[error("Cpu core of InstructionNumberInstant are inconsistent and cannot be subtracted")]
    InconsistentCore,
}
