//! This is only for reading `CpuInstructionNumber` specialization, not a complete package of [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
//!
//! Example:
//! ```ignore
//! use std::{fs, time::{Duration, Instant}, thread};
//! use cpu_instructions_reader::{InstructionNumber, InstructionNumberReader, InstructionNumberInstant};
//!
//! let reader = InstructionNumberReader::new().unwrap();
//! let record_1 = reader.instant(0).unwrap();
//!
//! thread::sleep(Duration::from_secs(1));
//!
//! let record_2 = reader.instant(0).unwrap();
//! let instructions = record_2 - record_1;
//!
//! println!("{instructions}");
//! ```
#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_panics_doc, clippy::module_name_repetitions)]
#![cfg(any(target_os = "linux", target_os = "android"))]
mod error;
pub mod ffi;
mod instant;
mod instruction_number;

use std::ptr;

use ffi::InstructionNumberReaderRaw;
use libc::{c_int, pid_t};

pub use error::{Error, Result};
pub use instant::InstructionNumberInstant;
pub use instruction_number::InstructionNumber;

#[derive(Debug)]
pub struct InstructionNumberReader {
    raw_ptr: *mut InstructionNumberReaderRaw,
}

impl Drop for InstructionNumberReader {
    fn drop(&mut self) {
        unsafe {
            ffi::disableInstructionNumberReader(self.raw_ptr);
            ffi::destroyInstructionNumberReader(self.raw_ptr);
        }
        self.raw_ptr = ptr::null_mut();
    }
}

impl InstructionNumberReader {
    /// pid: This measures the specified process/thread on any CPU. Set to `None` if measures all processes/threads on any cpu is wanted.
    ///
    /// # Errors
    ///
    /// If there is an error when calling the syscall, it will return an error
    pub fn new(pid: Option<pid_t>) -> Result<Self> {
        let cpus = c_int::try_from(num_cpus::get_physical())?;
        let cpus: Vec<_> = (0..cpus).collect();
        let cpus_ptr = cpus.as_ptr();

        let raw_ptr =
            unsafe { ffi::createInstructionNumberReader(cpus_ptr, cpus.len(), pid.unwrap_or(-1)) };

        if raw_ptr.is_null() {
            return Err(Error::FailedToCreate);
        }

        unsafe { ffi::enableInstructionNumberReader(raw_ptr) };

        Ok(Self { raw_ptr })
    }

    /// # Errors
    ///
    /// If there is an error when calling the syscall, it will return an error
    pub fn instant(&self, cpu: c_int) -> Result<InstructionNumberInstant> {
        let raw = unsafe { ffi::readInstructionNumberReader(self.raw_ptr, cpu) };

        if raw == -1 {
            Err(Error::FailedToRead)
        } else {
            let instructions = InstructionNumber::new(raw);
            let instant = InstructionNumberInstant::new(cpu, instructions);
            Ok(instant)
        }
    }
}
