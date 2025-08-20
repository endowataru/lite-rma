use std::future::Future;
use std::os::raw::c_int;
use std::os::raw::c_void;
use std::sync::Arc;

use crate::dev::mpi::communicator::Communicator;
use crate::dev::mpi::error::MpiError;
use crate::dev::mpi::send::SendOp;
use crate::dev::mpi::send::{SendDatatype, SendRequest};
use crate::util::ptr::{SendPtr, SendPtrMut};

pub type Rank = c_int;

pub trait Window: Sync {
    type Communicator: Communicator;
    fn comm(&self) -> &Self::Communicator;
    fn sched(&self) -> &<Self::Communicator as Communicator>::MpiSched {
        self.comm().sched()
    }

    unsafe fn create_dynamic(
        comm: Arc<Self::Communicator>,
    ) -> impl Future<Output = Result<Box<Self>, MpiError>> + Send;

    unsafe fn lock_all(&self, assert: c_int) -> impl Future<Output = Result<(), MpiError>> + Send;
    unsafe fn unlock_all(&self) -> impl Future<Output = Result<(), MpiError>> + Send;

    unsafe fn attach(
        &self,
        buf_ptr: SendPtrMut<c_void>,
        size: mpi::Address,
    ) -> impl Future<Output = Result<(), MpiError>> + Send;

    unsafe fn detach(
        &self,
        buf_ptr: SendPtrMut<c_void>,
    ) -> impl Future<Output = Result<(), MpiError>> + Send;

    unsafe fn win_lock_all(
        &self,
        assert: c_int,
    ) -> impl Future<Output = Result<(), MpiError>> + Send;
    unsafe fn win_unlock_all(&self) -> impl Future<Output = Result<(), MpiError>> + Send;

    unsafe fn rput(
        &self,
        origin_addr: SendPtr<c_void>,
        origin_count: c_int,
        origin_datatype: SendDatatype,
        target_rank: Rank,
        target_disp: mpi::Address,
        target_count: c_int,
        target_datatype: SendDatatype,
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send;

    unsafe fn rput_byte(
        &self,
        src_ptr: SendPtr<c_void>,
        dest_proc: c_int,
        dest_idx: mpi::Address,
        size: c_int,
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send {
        async move {
            unsafe {
                self.rput(
                    src_ptr,
                    size,
                    SendDatatype::new(mpi::ffi::RSMPI_INT8_T),
                    dest_proc,
                    dest_idx,
                    size,
                    SendDatatype::new(mpi::ffi::RSMPI_INT8_T),
                )
            }
            .await
        }
    }

    unsafe fn rget(
        &self,
        origin_addr: SendPtrMut<c_void>,
        origin_count: c_int,
        origin_datatype: SendDatatype,
        target_rank: c_int,
        target_disp: mpi::Address,
        target_count: c_int,
        target_datatype: SendDatatype,
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send;

    unsafe fn rget_byte(
        &self,
        dest_ptr: SendPtrMut<c_void>,
        src_proc: c_int,
        src_idx: mpi::Address,
        size: c_int,
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send {
        async move {
            unsafe {
                self.rget(
                    dest_ptr,
                    size,
                    SendDatatype::new(mpi::ffi::RSMPI_INT8_T),
                    src_proc,
                    src_idx,
                    size,
                    SendDatatype::new(mpi::ffi::RSMPI_INT8_T),
                )
            }
            .await
        }
    }

    unsafe fn compare_and_swap(
        &self,
        origin_addr: SendPtr<c_void>,
        compare_addr: SendPtr<c_void>,
        result_addr: SendPtrMut<c_void>,
        datatype: SendDatatype,
        target_rank: Rank,
        target_disp: mpi::Address,
    ) -> impl Future<Output = Result<(), MpiError>> + Send;

    unsafe fn fetch_and_op(
        &self,
        origin_addr: SendPtr<c_void>,
        result_addr: SendPtrMut<c_void>,
        datatype: SendDatatype,
        target_rank: Rank,
        target_disp: mpi::Address,
        op: SendOp,
    ) -> impl Future<Output = Result<(), MpiError>> + Send;

    fn flush(&self, rank: Rank) -> impl Future<Output = Result<(), MpiError>> + Send;
}
