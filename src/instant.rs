use std::ops::Sub;

use libc::c_int;

use crate::{InstructionNumber, Error, Result};

#[derive(Debug, Clone, Copy)]
pub struct InstructionNumberInstant {
    cpu: c_int,
    raw: InstructionNumber,
}

impl InstructionNumberInstant {
    pub(crate) const fn new(cpu: c_int, raw: InstructionNumber) -> Self {
        Self { cpu, raw }
    }

    /// Calculate the number of cpu instruction number between two recordings
    ///
    /// # Panics
    ///
    /// If the two records are not the same cpu, an error will be returned.
    #[must_use]
    pub fn instruction_number_since(&self, other: Self) -> InstructionNumber {
        self.instruction_number_since_checked(other).unwrap()
    }

    /// Calculate the number of cpu instruction number number between two recordings
    ///
    /// # Errors
    ///
    /// If the two records are not the same cpu, an error will be returned
    pub fn instruction_number_since_checked(&self, other: Self) -> Result<InstructionNumber> {
        if self.cpu == other.cpu {
            Ok(self.raw - other.raw)
        } else {
            Err(Error::InconsistentCore)
        }
    }
}

impl Sub for InstructionNumberInstant {
    type Output = InstructionNumber;

    fn sub(self, other: Self) -> Self::Output {
        self.instruction_number_since(other)
    }
}
