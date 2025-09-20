use core::ffi::c_void;
use std::ffi::c_int;
use std::future::Future;

use crate::dev::mpi::datatype::get_datatype_from_size;
use crate::dev::mpi::error::MpiError;
use crate::dev::mpi::mpi_sched::MpiSched;
use crate::dev::mpi::send::SendDatatype;
use crate::dev::mpi::window::Window;
use crate::rma::{
    self, RmaLocalPtr, RmaLocalPtrMut, RmaProcRemotePtr, RmaProcRemotePtrMut, RmaRemotePtr,
    RmaRemotePtrMut,
};
use crate::rma::{RmaLocalAttach, RmaPointable};
use crate::traits::ComBaseDevice;
use crate::util::ptr::{SendPtr, SendPtrMut};

pub trait MpiRmaDevice: Sync + Send {
    type Window: Window;
    fn window(&self) -> &Self::Window;
}

impl<D: MpiRmaDevice + ComBaseDevice<Error = MpiError>> rma::RmaDevice for D {
    type LocalKey = ();
    type RemoteKey = ();

    type LocalPtr<T: RmaPointable> = rma::SimpleLocalPtr<T, Self>;
    type LocalPtrMut<T: RmaPointable> = rma::SimpleLocalPtrMut<T, Self>;
    type RemotePtr<T: RmaPointable> = rma::SimpleRemotePtr<T, Self>;
    type RemotePtrMut<T: RmaPointable> = rma::SimpleRemotePtrMut<T, Self>;
    type ProcRemotePtr<T: RmaPointable> = rma::SimpleProcRemotePtr<T, Self>;
    type ProcRemotePtrMut<T: RmaPointable> = rma::SimpleProcRemotePtrMut<T, Self>;
    type LocalAttach<T: RmaPointable> = rma::SimpleLocalAttach<T, Self>;

    unsafe fn attach_void(
        &self,
        ptr: SendPtrMut<c_void>,
        size: usize,
    ) -> impl Future<Output = Result<Self::LocalAttach<c_void>, Self::Error>> + Send {
        async move {
            unsafe { self.window().attach(ptr, size as mpi::Address).await }?;
            Ok(unsafe { Self::LocalAttach::new(ptr.get(), (), ()) })
        }
    }

    unsafe fn detach_void(
        &self,
        mut la: Self::LocalAttach<c_void>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let raw_ptr = la.lptr_mut().ptr_mut();
            unsafe { la.reset() };
            unsafe { self.window().detach(SendPtrMut::new(raw_ptr)).await }
        }
    }

    unsafe fn write_void(
        &self,
        src_lptr: Self::LocalPtr<c_void>,
        dest_rptr: Self::ProcRemotePtrMut<c_void>,
        size: usize,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let req = unsafe {
                self.window().rput_byte(
                    SendPtr::new(src_lptr.ptr()),
                    dest_rptr.proc() as c_int,
                    dest_rptr.rptr_mut().ptr_mut().addr() as mpi::Address,
                    size as c_int,
                )
            }
            .await?;
            unsafe { self.window().sched().wait(req) }.await
        }
    }

    unsafe fn buf_write_void(
        &self,
        src_ptr: SendPtr<c_void>,
        dest_rptr: Self::ProcRemotePtrMut<c_void>,
        size: usize,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let req = unsafe {
                self.window().rput_byte(
                    src_ptr,
                    dest_rptr.proc() as c_int,
                    dest_rptr.rptr_mut().ptr_mut().addr() as mpi::Address,
                    size as c_int,
                )
            }
            .await?;
            unsafe { self.window().sched().wait(req) }.await
        }
    }

    unsafe fn read_void(
        &self,
        src_rptr: Self::ProcRemotePtr<c_void>,
        dest_lptr: Self::LocalPtrMut<c_void>,
        size: usize,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let req = unsafe {
                self.window().rget_byte(
                    SendPtrMut::new(dest_lptr.ptr_mut()),
                    src_rptr.proc() as c_int,
                    src_rptr.rptr().ptr().addr() as mpi::Address,
                    size as c_int,
                )
            }
            .await?;
            unsafe { self.window().sched().wait(req) }.await
        }
    }

    unsafe fn compare_and_swap<T: RmaPointable>(
        &self,
        target_rptr: Self::ProcRemotePtrMut<T>,
        expected: T,
        desired: T,
    ) -> impl Future<Output = Result<T, Self::Error>> + Send {
        async move {
            let mut result = std::mem::MaybeUninit::<T>::uninit();
            let datatype = SendDatatype::new(get_datatype_from_size::<T>());
            unsafe {
                self.window().compare_and_swap(
                    SendPtr::new(&desired as *const T as *const c_void),
                    SendPtr::new(&expected as *const T as *const c_void),
                    SendPtrMut::new(result.as_mut_ptr() as *mut c_void),
                    datatype,
                    target_rptr.proc() as c_int,
                    target_rptr.rptr_mut().ptr_mut().addr() as mpi::Address,
                )
            }
            .await?;
            Ok(unsafe { result.assume_init() })
        }
    }

    unsafe fn atomic_read<T: RmaPointable>(
        &self,
        src_rptr: Self::ProcRemotePtr<T>,
    ) -> impl Future<Output = Result<T, Self::Error>> + Send {
        async move {
            let mut result = std::mem::MaybeUninit::<T>::uninit();
            // TODO: Use MPI_Fetch_and_op() for atomicity
            unsafe { self.read(src_rptr, Self::LocalPtrMut::new(result.as_mut_ptr(), ()), 1) }
                .await?;
            Ok(unsafe { result.assume_init() })
        }
    }

    unsafe fn atomic_write<T: RmaPointable>(
        &self,
        dest_rptr: Self::ProcRemotePtrMut<T>,
        src: T,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            // TODO: Use MPI_Fetch_and_op() for atomicity
            unsafe {
                self.write(
                    Self::LocalPtr::new(&src as *const T as *mut T, ()),
                    dest_rptr,
                    1,
                )
            }
            .await
        }
    }
}
