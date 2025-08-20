use std::{
    ffi::{c_int, c_void},
    future::Future,
    mem::size_of,
};

use mpi::datatype::AsDatatype;
use mpi::{raw::AsRaw, traits::Equivalence};

use crate::{
    coll::traits::CollDevice,
    dev::mpi::{
        communicator::Communicator,
        error::MpiError,
        mpi_sched::MpiSched,
        send::{SendDatatype, SendOp},
    },
    traits::ComBaseDevice,
    util::ptr::{SendPtr, SendPtrMut},
};

pub trait MpiCollDevice: Sync + Send {
    type Communicator: Communicator;
    fn comm(&self) -> &Self::Communicator;
}

impl<D: MpiCollDevice + ComBaseDevice> CollDevice for D {
    type Error = MpiError;

    fn barrier(&self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let req = unsafe { self.comm().ibarrier() }.await?;
            unsafe { self.comm().sched().wait(req) }.await?;
            Ok(())
        }
    }

    fn allgather<T>(
        &self,
        send_buf: &[T],
        recv_buf: &mut [T],
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        T: Copy + Send + Sync,
    {
        async move {
            assert!(send_buf.len() * self.num_procs() >= recv_buf.len());
            let count = (send_buf.len() * size_of::<T>()) as c_int;
            let datatype = SendDatatype::new(unsafe { mpi::ffi::RSMPI_UINT8_T });
            let req = unsafe {
                self.comm().iallgather(
                    SendPtr::new(send_buf.as_ptr() as *const c_void),
                    count,
                    datatype,
                    SendPtrMut::new(recv_buf.as_mut_ptr() as *mut c_void),
                    count,
                    datatype,
                )
            }
            .await?;
            unsafe { self.comm().sched().wait(req) }.await?;
            Ok(())
        }
    }

    fn allreduce<T>(
        &self,
        send_buf: &[T],
        recv_buf: &mut [T],
        op: crate::coll::traits::SystemOperation,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        T: Copy + Send + Sync + Equivalence,
    {
        let op = SendOp::new(op.as_raw());
        async move {
            assert_eq!(send_buf.len(), recv_buf.len());
            let count = (send_buf.len() * size_of::<T>()) as c_int;
            let datatype = SendDatatype::new(send_buf.as_datatype().as_raw());
            let req = unsafe {
                self.comm().iallreduce(
                    SendPtr::new(send_buf.as_ptr() as *const c_void),
                    SendPtrMut::new(recv_buf.as_mut_ptr() as *mut c_void),
                    count,
                    datatype,
                    op,
                )
            }
            .await?;
            unsafe { self.comm().sched().wait(req) }.await?;
            Ok(())
        }
    }
}
