use std::future::Future;
use std::os::raw::{c_int, c_void};
use std::sync::Arc;

use crate::dev::mpi::communicator::Communicator;
use crate::dev::mpi::error::MpiError;
use crate::dev::mpi::send::{SendDatatype, SendOp, SendRequest, SendWin};
use crate::dev::mpi::window::{Rank, Window};
use crate::util::ptr::{SendPtr, SendPtrMut};

pub struct DirectWindow<C: Communicator> {
    comm: Arc<C>,
    win: SendWin,
}

impl<C: Communicator> Window for DirectWindow<C> {
    type Communicator = C;
    fn comm(&self) -> &Self::Communicator {
        self.comm.as_ref()
    }

    unsafe fn create_dynamic(
        comm: Arc<Self::Communicator>,
    ) -> impl Future<Output = Result<Box<Self>, MpiError>> + Send {
        async move {
            let mut win = std::mem::MaybeUninit::uninit();
            check_mpi_error(unsafe {
                mpi::ffi::MPI_Win_create_dynamic(
                    mpi::ffi::RSMPI_INFO_NULL,
                    comm.get(),
                    win.as_mut_ptr(),
                )
            })?;
            Ok(Box::new(DirectWindow {
                comm,
                win: SendWin::new(unsafe { win.assume_init() }),
            }))
        }
    }

    unsafe fn lock_all(&self, assert: c_int) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move { check_mpi_error(unsafe { mpi::ffi::MPI_Win_lock_all(assert, self.win.get()) }) }
    }
    unsafe fn unlock_all(&self) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move { check_mpi_error(unsafe { mpi::ffi::MPI_Win_unlock_all(self.win.get()) }) }
    }

    unsafe fn attach(
        &self,
        buf_ptr: SendPtrMut<c_void>,
        size: mpi::Address,
    ) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move {
            check_mpi_error(unsafe {
                mpi::ffi::MPI_Win_attach(self.win.get(), buf_ptr.get(), size)
            })
        }
    }

    unsafe fn detach(
        &self,
        buf_ptr: SendPtrMut<c_void>,
    ) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move {
            check_mpi_error(unsafe { mpi::ffi::MPI_Win_detach(self.win.get(), buf_ptr.get()) })
        }
    }

    unsafe fn win_lock_all(
        &self,
        assert: c_int,
    ) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move { check_mpi_error(unsafe { mpi::ffi::MPI_Win_lock_all(assert, self.win.get()) }) }
    }

    unsafe fn win_unlock_all(&self) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move { check_mpi_error(unsafe { mpi::ffi::MPI_Win_unlock_all(self.win.get()) }) }
    }

    unsafe fn rput(
        &self,
        origin_addr: SendPtr<c_void>,
        origin_count: c_int,
        origin_datatype: SendDatatype,
        target_rank: Rank,
        target_disp: mpi::Address,
        target_count: c_int,
        target_datatype: SendDatatype,
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send {
        async move {
            let mut req = std::mem::MaybeUninit::uninit();
            check_mpi_error(unsafe {
                mpi::ffi::MPI_Rput(
                    origin_addr.get(),
                    origin_count,
                    origin_datatype.get(),
                    target_rank,
                    target_disp,
                    target_count,
                    target_datatype.get(),
                    self.win.get(),
                    req.as_mut_ptr(),
                )
            })?;
            Ok(SendRequest::new(unsafe { req.assume_init() }))
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
    ) -> impl Future<Output = Result<SendRequest, MpiError>> + Send {
        async move {
            let mut req = std::mem::MaybeUninit::uninit();
            check_mpi_error(unsafe {
                mpi::ffi::MPI_Rget(
                    origin_addr.get(),
                    origin_count,
                    origin_datatype.get(),
                    target_rank,
                    target_disp,
                    target_count,
                    target_datatype.get(),
                    self.win.get(),
                    req.as_mut_ptr(),
                )
            })?;
            Ok(SendRequest::new(unsafe { req.assume_init() }))
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
    ) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move {
            check_mpi_error(unsafe {
                mpi::ffi::MPI_Compare_and_swap(
                    origin_addr.get(),
                    compare_addr.get(),
                    result_addr.get(),
                    datatype.get(),
                    target_rank,
                    target_disp,
                    self.win.get(),
                )
            })
        }
    }

    unsafe fn fetch_and_op(
        &self,
        origin_addr: SendPtr<c_void>,
        result_addr: SendPtrMut<c_void>,
        datatype: SendDatatype,
        target_rank: Rank,
        target_disp: mpi::Address,
        op: SendOp,
    ) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move {
            check_mpi_error(unsafe {
                mpi::ffi::MPI_Fetch_and_op(
                    origin_addr.get(),
                    result_addr.get(),
                    datatype.get(),
                    target_rank,
                    target_disp,
                    op.get(),
                    self.win.get(),
                )
            })
        }
    }

    fn flush(&self, rank: Rank) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move { check_mpi_error(unsafe { mpi::ffi::MPI_Win_flush(rank, self.win.get()) }) }
    }
}

fn is_mpi_success(ret: c_int) -> bool {
    ret == (mpi::ffi::MPI_SUCCESS as c_int)
}

fn check_mpi_error(ret: c_int) -> Result<(), MpiError> {
    if is_mpi_success(ret) {
        Ok(())
    } else {
        Err(MpiError::new(ret))
    }
}
