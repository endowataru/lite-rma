use std::{error, fmt};

#[derive(Debug)]
pub struct MpiError {
    err: mpi::Error,
}

impl MpiError {
    pub fn new(err: mpi::Error) -> Self {
        Self { err }
    }

    pub fn check(ret: mpi::Error) -> Result<(), MpiError> {
        if is_mpi_success(ret) {
            Ok(())
        } else {
            Err(MpiError::new(ret))
        }
    }
}

fn is_mpi_success(ret: mpi::Error) -> bool {
    ret == (mpi::ffi::MPI_SUCCESS as mpi::Error)
}

impl fmt::Display for MpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = [0i8; mpi::ffi::MPI_MAX_ERROR_STRING as usize];
        let mut len = 0i32;

        unsafe {
            mpi::ffi::MPI_Error_string(self.err, buffer.as_mut_ptr(), &mut len);
        }

        let error_string = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()).to_string_lossy() };

        write!(f, "{}", error_string)
    }
}

impl error::Error for MpiError {}
