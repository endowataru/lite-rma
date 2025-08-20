use std::ffi::{c_int, c_void};

use crate::{
    dev::mpi::{
        error::MpiError,
        mpi_sched::MpiSched,
        send::{SendDatatype, SendOp, SendRequest},
    },
    util::ptr::{SendPtr, SendPtrMut},
};

pub trait Communicator: Sync + Send {
    type MpiSched: MpiSched;
    fn sched(&self) -> &Self::MpiSched;

    fn get(&self) -> mpi::ffi::MPI_Comm;

    fn rank(&self) -> c_int;
    fn size(&self) -> c_int;

    unsafe fn ibarrier(&self) -> impl Future<Output = Result<SendRequest, MpiError>> + Send;

    unsafe fn iallgather(
        &self,
        send_buf: SendPtr<c_void>,
        send_count: c_int,
        send_type: SendDatatype,
        recv_buf: SendPtrMut<c_void>,
        recv_count: c_int,
        recv_type: SendDatatype,
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send;

    unsafe fn iallreduce(
        &self,
        send_buf: SendPtr<c_void>,
        recv_buf: SendPtrMut<c_void>,
        count: c_int,
        datatype: SendDatatype,
        op: SendOp,
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send;
}
