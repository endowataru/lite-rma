use crate::{
    ProcInt,
    traits::ComBaseDevice,
    util::ptr::{SendPtr, SendPtrMut},
};

use core::ffi::c_void;

use std::fmt;

pub trait RmaPointable: Send + Sync + fmt::Debug {}
impl<T> RmaPointable for T where T: ?Sized + Send + Sync + fmt::Debug {}

pub trait RmaPtr<T: RmaPointable, D: RmaDevice + ?Sized>:
    Sized + Copy + Send + Sync + Default + fmt::Debug
{
    unsafe fn offset(self, count: isize) -> Self
    where
        T: Sized;

    unsafe fn add(self, count: usize) -> Self
    where
        T: Sized,
    {
        unsafe { self.offset(count as isize) }
    }
    unsafe fn sub(self, count: usize) -> Self
    where
        T: Sized,
    {
        unsafe { self.offset((count as isize).wrapping_neg()) }
    }
}

pub trait RmaLocalPtr<T: RmaPointable, D: RmaDevice + ?Sized>: RmaPtr<T, D> {
    fn new(ptr: *const T, key: D::LocalKey) -> Self;
    fn cast<U: RmaPointable>(&self) -> D::LocalPtr<U>;
    fn as_mut(&self) -> D::LocalPtrMut<T>;

    fn ptr(&self) -> *const T;
    fn lkey(&self) -> D::LocalKey;
}
pub trait RmaLocalPtrMut<T: RmaPointable, D: RmaDevice + ?Sized>: RmaPtr<T, D> {
    fn new(ptr: *mut T, key: D::LocalKey) -> Self;
    fn cast<U: RmaPointable>(&self) -> D::LocalPtrMut<U>;
    fn as_const(&self) -> D::LocalPtr<T>;

    fn ptr(&self) -> *const T;
    fn ptr_mut(&self) -> *mut T;
    fn lkey(&self) -> D::LocalKey;
}
pub trait RmaRemotePtr<T: RmaPointable, D: RmaDevice + ?Sized>: RmaPtr<T, D> {
    fn new(ptr: *const T, key: D::RemoteKey) -> Self;
    fn cast<U: RmaPointable>(&self) -> D::RemotePtr<U>;

    unsafe fn ptr(&self) -> *const T;
    fn rkey(&self) -> D::RemoteKey;
}
pub trait RmaRemotePtrMut<T: RmaPointable, D: RmaDevice + ?Sized>: RmaPtr<T, D> {
    fn new(ptr: *mut T, key: D::RemoteKey) -> Self;
    fn cast<U: RmaPointable>(&self) -> D::RemotePtrMut<U>;
    fn as_const(&self) -> D::RemotePtr<T>;

    unsafe fn ptr(&self) -> *const T;
    unsafe fn ptr_mut(&self) -> *mut T;
    fn rkey(&self) -> D::RemoteKey;
}
pub trait RmaProcRemotePtr<T: RmaPointable, D: RmaDevice + ?Sized>: RmaPtr<T, D> {
    fn new(proc: ProcInt, rptr: D::RemotePtr<T>) -> Self;
    fn cast<U: RmaPointable>(&self) -> D::ProcRemotePtr<U>;

    fn proc(&self) -> ProcInt;
    fn rptr(&self) -> D::RemotePtr<T>;
    unsafe fn ptr(&self) -> *const T;
    fn rkey(&self) -> D::RemoteKey;
}
pub trait RmaProcRemotePtrMut<T: RmaPointable, D: RmaDevice + ?Sized>: RmaPtr<T, D> {
    fn new(proc: ProcInt, rptr: D::RemotePtrMut<T>) -> Self;
    fn cast<U: RmaPointable>(&self) -> D::ProcRemotePtrMut<U>;

    fn proc(&self) -> ProcInt;
    fn rptr(&self) -> D::RemotePtr<T>;
    fn rptr_mut(&self) -> D::RemotePtrMut<T>;
    unsafe fn ptr(&self) -> *const T;
    unsafe fn ptr_mut(&self) -> *mut T;
    fn rkey(&self) -> D::RemoteKey;
}

pub trait RmaLocalAttach<T: RmaPointable, D: RmaDevice + ?Sized>:
    Sized + Send + Sync + Default + fmt::Debug
{
    unsafe fn new(ptr: *mut T, lkey: D::LocalKey, rkey: D::RemoteKey) -> Self;
    fn cast<U: RmaPointable>(self) -> D::LocalAttach<U>;
    fn lptr(&self) -> D::LocalPtr<T>;
    fn lptr_mut(&self) -> D::LocalPtrMut<T>;
    fn rptr(&self) -> D::RemotePtr<T>;
    fn rptr_mut(&self) -> D::RemotePtrMut<T>;
}

pub trait RmaDevice: ComBaseDevice {
    type Error: std::error::Error + Send;

    type LocalKey: Copy + Send + Sync + Default + fmt::Debug;
    type RemoteKey: Copy + Send + Sync + Default + fmt::Debug;

    type LocalPtr<T: RmaPointable>: RmaLocalPtr<T, Self>;
    type LocalPtrMut<T: RmaPointable>: RmaLocalPtrMut<T, Self>;
    type RemotePtr<T: RmaPointable>: RmaRemotePtr<T, Self>;
    type RemotePtrMut<T: RmaPointable>: RmaRemotePtrMut<T, Self>;
    type ProcRemotePtr<T: RmaPointable>: RmaProcRemotePtr<T, Self>;
    type ProcRemotePtrMut<T: RmaPointable>: RmaProcRemotePtrMut<T, Self>;
    type LocalAttach<T: RmaPointable>: RmaLocalAttach<T, Self>;

    unsafe fn attach_void(
        &self,
        ptr: SendPtrMut<c_void>,
        size: usize,
    ) -> impl Future<Output = Result<Self::LocalAttach<c_void>, Self::Error>> + Send;

    unsafe fn attach<T: RmaPointable>(
        &self,
        ptr: SendPtrMut<T>,
        size: usize,
    ) -> impl Future<Output = Result<Self::LocalAttach<T>, Self::Error>> + Send {
        async move {
            let ret =
                unsafe { self.attach_void(ptr.cast::<c_void>(), size * size_of::<T>()) }.await?;
            Ok(ret.cast::<T>())
        }
    }

    unsafe fn detach_void(
        &self,
        ptr: Self::LocalAttach<c_void>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    unsafe fn detach<T: RmaPointable>(
        &self,
        la: Self::LocalAttach<T>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move { unsafe { self.detach_void(la.cast::<c_void>()) }.await }
    }

    unsafe fn write_void(
        &self,
        src_lptr: Self::LocalPtr<c_void>,
        dest_rptr: Self::ProcRemotePtrMut<c_void>,
        size: usize,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    unsafe fn write<T: RmaPointable>(
        &self,
        src_lptr: Self::LocalPtr<T>,
        dest_rptr: Self::ProcRemotePtrMut<T>,
        size: usize,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            unsafe {
                self.write_void(
                    src_lptr.cast::<c_void>(),
                    dest_rptr.cast::<c_void>(),
                    size * size_of::<T>(),
                )
            }
            .await
        }
    }

    unsafe fn buf_write_void(
        &self,
        src_ptr: SendPtr<c_void>,
        dest_rptr: Self::ProcRemotePtrMut<c_void>,
        size: usize,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    unsafe fn buf_write<T: RmaPointable>(
        &self,
        src_ptr: SendPtr<T>,
        dest_rptr: Self::ProcRemotePtrMut<T>,
        size: usize,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            unsafe { self.buf_write_void(src_ptr.cast(), dest_rptr.cast(), size * size_of::<T>()) }
                .await
        }
    }

    unsafe fn read_void(
        &self,
        src_rptr: Self::ProcRemotePtr<c_void>,
        dest_lptr: Self::LocalPtrMut<c_void>,
        size: usize,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    unsafe fn read<T: RmaPointable>(
        &self,
        src_rptr: Self::ProcRemotePtr<T>,
        dest_lptr: Self::LocalPtrMut<T>,
        size: usize,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            unsafe {
                self.read_void(
                    src_rptr.cast::<c_void>(),
                    dest_lptr.cast::<c_void>(),
                    size * size_of::<T>(),
                )
            }
            .await
        }
    }

    unsafe fn compare_and_swap<T: RmaPointable>(
        &self,
        target_rptr: Self::ProcRemotePtrMut<T>,
        expected: T,
        desired: T,
    ) -> impl Future<Output = Result<T, Self::Error>> + Send;

    unsafe fn atomic_read<T: RmaPointable>(
        &self,
        src_rptr: Self::ProcRemotePtr<T>,
    ) -> impl Future<Output = Result<T, Self::Error>> + Send;

    unsafe fn atomic_write<T: RmaPointable>(
        &self,
        dest_rptr: Self::ProcRemotePtrMut<T>,
        src: T,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
