use super::communicator::Communicator;
use crate::{
    dev::mpi::{
        error::MpiError,
        mpi_sched::MpiSched,
        send::{SendComm, SendDatatype, SendOp, SendRequest},
    },
    util::ptr::{SendPtr, SendPtrMut},
};
use mpi::ffi::MPI_Comm;
use std::{ffi::c_void, future::Future, sync::Arc};

pub struct DirectCommunicator<S: MpiSched> {
    sched: Arc<S>,
    comm: SendComm,
}

impl<S: MpiSched> DirectCommunicator<S> {
    pub fn new(sched: Arc<S>, comm: MPI_Comm) -> Self {
        Self {
            sched,
            comm: SendComm::new(comm),
        }
    }
}

impl<S: MpiSched> Communicator for DirectCommunicator<S> {
    type MpiSched = S;
    fn sched(&self) -> &Self::MpiSched {
        self.sched.as_ref()
    }

    fn get(&self) -> mpi::ffi::MPI_Comm {
        self.comm.get()
    }

    fn rank(&self) -> std::ffi::c_int {
        let mut rank = 0;
        unsafe { mpi::ffi::MPI_Comm_rank(self.comm.get(), &mut rank) };
        rank
    }
    fn size(&self) -> std::ffi::c_int {
        let mut size = 0;
        unsafe { mpi::ffi::MPI_Comm_size(self.comm.get(), &mut size) };
        size
    }

    unsafe fn ibarrier(&self) -> impl Future<Output = Result<SendRequest, MpiError>> + Send {
        async move {
            let mut req = std::mem::MaybeUninit::uninit();
            MpiError::check(unsafe { mpi::ffi::MPI_Ibarrier(self.comm.get(), req.as_mut_ptr()) })?;
            Ok(SendRequest::new(unsafe { req.assume_init() }))
        }
    }

    unsafe fn iallgather(
        &self,
        send_buf: SendPtr<c_void>,
        send_count: std::ffi::c_int,
        send_type: SendDatatype,
        recv_buf: SendPtrMut<c_void>,
        recv_count: std::ffi::c_int,
        recv_type: SendDatatype,
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send {
        async move {
            let mut req = std::mem::MaybeUninit::uninit();
            MpiError::check(unsafe {
                mpi::ffi::MPI_Iallgather(
                    send_buf.get(),
                    send_count,
                    send_type.get(),
                    recv_buf.get(),
                    recv_count,
                    recv_type.get(),
                    self.comm.get(),
                    req.as_mut_ptr(),
                )
            })?;
            Ok(SendRequest::new(unsafe { req.assume_init() }))
        }
    }

    unsafe fn iallreduce(
        &self,
        send_buf: SendPtr<c_void>,
        recv_buf: SendPtrMut<c_void>,
        count: std::ffi::c_int,
        datatype: SendDatatype,
        op: SendOp,
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send {
        async move {
            let mut req = std::mem::MaybeUninit::uninit();
            MpiError::check(unsafe {
                mpi::ffi::MPI_Iallreduce(
                    send_buf.get(),
                    recv_buf.get(),
                    count,
                    datatype.get(),
                    op.get(),
                    self.comm.get(),
                    req.as_mut_ptr(),
                )
            })?;
            Ok(SendRequest::new(unsafe { req.assume_init() }))
        }
    }
}
