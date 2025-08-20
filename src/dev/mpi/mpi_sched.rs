use std::ffi::c_int;

use crate::{
    dev::mpi::{error::MpiError, send::SendRequest},
    ult::sched::Sched,
    util::ptr::SendPtrMut,
};

pub trait MpiSched: Sched {
    unsafe fn test(
        &self,
        req: SendRequest,
        status: SendPtrMut<mpi::ffi::MPI_Status>,
    ) -> impl Future<Output = Result<c_int, MpiError>> + Send;

    unsafe fn test_status_ignore(
        &self,
        req: SendRequest,
    ) -> impl Future<Output = Result<c_int, MpiError>> + Send {
        unsafe { self.test(req, SendPtrMut::new(mpi::ffi::RSMPI_STATUS_IGNORE)) }
    }

    unsafe fn wait(&self, req: SendRequest) -> impl Future<Output = Result<(), MpiError>> + Send;
}
