//! Base bindings to c code
//!
//! Unless you are really sure you have to use them, just use [`crate::InstructionNumberReader`] for normal purpose

use libc::{c_int, c_longlong as c_ll, pid_t, size_t};

extern "C" {
    /// Mock constructor
    ///
    /// This C function internally calls malloc to allocate a memory to construct InstructionNumberReaderRaw, and returns a pointer to the heap
    ///
    /// If there is an error in the creation process, the memory will be free, and the pointer will be set to NULL to return, special attention should be paid
    pub fn createInstructionNumberReader(cpus: *const c_int, num_cpus: size_t, pid: pid_t) -> *mut InstructionNumberReaderRaw;

    /// Mock destructor
    ///
    /// This C function internally calls free to release the memory and sets the pointer to NULL
    pub fn destroyInstructionNumberReader(reader: *mut InstructionNumberReaderRaw);

    /// This C function calls ioctl to start recording instruction number, see [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
    pub fn enableInstructionNumberReader(reader: *mut InstructionNumberReaderRaw);

    /// This C function calls ioctl to stop recording instruction number, see [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
    pub fn disableInstructionNumberReader(reader: *mut InstructionNumberReaderRaw);

    /// This C function reads instruction number information by reading the file identifier, see [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
    ///
    /// The returned array pointer is also allocated by malloc, consider calling [`libc::free`] to release to ensure memory safety, and remember to prevent dangling pointers
    ///
    /// NOTE: The length of the array is the number of CPUs used during construction. Consider using the `size` member of [`self::InstructionNumberReaderRaw`] to determine the length of the array
    pub fn readInstructionNumberReader(reader: *mut InstructionNumberReaderRaw, cpu: c_int) -> c_ll;
}

/// The Raw Reader Structure, corresponding to the same structure in C
#[repr(C)]
pub struct InstructionNumberReaderRaw {
    pub size: size_t,
    cpus: *mut c_int,
}
