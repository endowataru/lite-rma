use super::traits::*;

use crate::util::ptr::{SendPtr, SendPtrMut, UndropPtr};

use derive_where::derive_where;

#[derive_where(Debug, Copy, Clone, Default)]
pub struct SimpleLocalPtr<T: RmaPointable, D: RmaDevice + ?Sized> {
    ptr: SendPtr<T>,
    lkey: D::LocalKey,
}

#[derive_where(Debug, Copy, Clone, Default)]
pub struct SimpleLocalPtrMut<T: RmaPointable, D: RmaDevice + ?Sized> {
    ptr: SendPtrMut<T>,
    lkey: D::LocalKey,
}

#[derive_where(Debug, Copy, Clone, Default)]
pub struct SimpleRemotePtr<T: RmaPointable, D: RmaDevice + ?Sized> {
    ptr: SendPtr<T>,
    rkey: D::RemoteKey,
}

#[derive_where(Debug, Copy, Clone, Default)]
pub struct SimpleRemotePtrMut<T: RmaPointable, D: RmaDevice + ?Sized> {
    ptr: SendPtrMut<T>,
    rkey: D::RemoteKey,
}

#[derive_where(Debug, Copy, Clone, Default)]
pub struct SimpleProcRemotePtr<T: RmaPointable, D: RmaDevice + ?Sized> {
    proc: crate::ProcInt,
    rptr: D::RemotePtr<T>,
}

#[derive_where(Debug, Copy, Clone, Default)]
pub struct SimpleProcRemotePtrMut<T: RmaPointable, D: RmaDevice + ?Sized> {
    proc: crate::ProcInt,
    rptr: D::RemotePtrMut<T>,
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaLocalPtr<T, D> for SimpleLocalPtr<T, D> {
    fn new(ptr: *const T, lkey: D::LocalKey) -> Self {
        Self {
            ptr: SendPtr::new(ptr),
            lkey,
        }
    }
    fn cast<U: RmaPointable>(&self) -> D::LocalPtr<U> {
        RmaLocalPtr::new(self.ptr.cast::<U>().get(), self.lkey)
    }
    fn as_mut(&self) -> D::LocalPtrMut<T> {
        D::LocalPtrMut::<T>::new(self.ptr.get().cast_mut(), self.lkey)
    }

    fn ptr(&self) -> *const T {
        self.ptr.get()
    }
    fn lkey(&self) -> D::LocalKey {
        self.lkey
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaPtr<T, D> for SimpleLocalPtr<T, D> {
    unsafe fn offset(self, count: isize) -> Self {
        Self::new(unsafe { self.ptr.offset(count) }.get(), self.lkey)
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaLocalPtrMut<T, D> for SimpleLocalPtrMut<T, D> {
    fn new(ptr: *mut T, lkey: D::LocalKey) -> Self {
        Self {
            ptr: SendPtrMut::new(ptr),
            lkey,
        }
    }
    fn cast<U: RmaPointable>(&self) -> D::LocalPtrMut<U> {
        RmaLocalPtrMut::new(self.ptr.cast::<U>().get(), self.lkey)
    }
    fn as_const(&self) -> D::LocalPtr<T> {
        D::LocalPtr::<T>::new(self.ptr.get(), self.lkey)
    }
    fn ptr(&self) -> *const T {
        self.ptr.get()
    }
    fn ptr_mut(&self) -> *mut T {
        self.ptr.get()
    }
    fn lkey(&self) -> D::LocalKey {
        self.lkey
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaPtr<T, D> for SimpleLocalPtrMut<T, D> {
    unsafe fn offset(self, count: isize) -> Self {
        Self::new(unsafe { self.ptr.offset(count) }.get(), self.lkey)
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaRemotePtr<T, D> for SimpleRemotePtr<T, D> {
    fn new(ptr: *const T, rkey: D::RemoteKey) -> Self {
        Self {
            ptr: SendPtr::new(ptr),
            rkey,
        }
    }
    fn cast<U: RmaPointable>(&self) -> D::RemotePtr<U> {
        RmaRemotePtr::new(self.ptr.cast::<U>().get(), self.rkey)
    }
    unsafe fn ptr(&self) -> *const T {
        self.ptr.get()
    }
    fn rkey(&self) -> D::RemoteKey {
        self.rkey
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaPtr<T, D> for SimpleRemotePtr<T, D> {
    unsafe fn offset(self, count: isize) -> Self {
        Self::new(unsafe { self.ptr.offset(count) }.get(), self.rkey)
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaRemotePtrMut<T, D> for SimpleRemotePtrMut<T, D> {
    fn new(ptr: *mut T, rkey: D::RemoteKey) -> Self {
        Self {
            ptr: SendPtrMut::new(ptr),
            rkey,
        }
    }
    fn cast<U: RmaPointable>(&self) -> D::RemotePtrMut<U> {
        RmaRemotePtrMut::new(self.ptr.cast::<U>().get(), self.rkey)
    }
    fn as_const(&self) -> D::RemotePtr<T> {
        D::RemotePtr::<T>::new(self.ptr.get(), self.rkey)
    }
    unsafe fn ptr(&self) -> *const T {
        self.ptr.get()
    }
    unsafe fn ptr_mut(&self) -> *mut T {
        self.ptr.get()
    }
    fn rkey(&self) -> D::RemoteKey {
        self.rkey
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaPtr<T, D> for SimpleRemotePtrMut<T, D> {
    unsafe fn offset(self, count: isize) -> Self {
        Self::new(unsafe { self.ptr.offset(count) }.get(), self.rkey)
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaProcRemotePtr<T, D> for SimpleProcRemotePtr<T, D> {
    fn new(proc: crate::ProcInt, rptr: D::RemotePtr<T>) -> Self {
        Self { proc, rptr }
    }
    fn cast<U: RmaPointable>(&self) -> D::ProcRemotePtr<U> {
        RmaProcRemotePtr::new(self.proc, self.rptr.cast::<U>())
    }
    fn proc(&self) -> crate::ProcInt {
        self.proc
    }
    fn rptr(&self) -> D::RemotePtr<T> {
        self.rptr
    }
    unsafe fn ptr(&self) -> *const T {
        unsafe { self.rptr.ptr() }
    }
    fn rkey(&self) -> D::RemoteKey {
        self.rptr.rkey()
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaPtr<T, D> for SimpleProcRemotePtr<T, D> {
    unsafe fn offset(self, count: isize) -> Self {
        Self {
            proc: self.proc,
            rptr: unsafe { self.rptr.offset(count) },
        }
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaProcRemotePtrMut<T, D>
    for SimpleProcRemotePtrMut<T, D>
{
    fn new(proc: crate::ProcInt, rptr: D::RemotePtrMut<T>) -> Self {
        Self { proc, rptr }
    }
    fn cast<U: RmaPointable>(&self) -> D::ProcRemotePtrMut<U> {
        RmaProcRemotePtrMut::new(self.proc, self.rptr.cast::<U>())
    }
    fn proc(&self) -> crate::ProcInt {
        self.proc
    }
    fn rptr(&self) -> D::RemotePtr<T> {
        self.rptr.as_const()
    }
    fn rptr_mut(&self) -> D::RemotePtrMut<T> {
        self.rptr
    }
    unsafe fn ptr(&self) -> *const T {
        unsafe { self.rptr.ptr() }
    }
    unsafe fn ptr_mut(&self) -> *mut T {
        unsafe { self.rptr.ptr_mut() }
    }
    fn rkey(&self) -> D::RemoteKey {
        self.rptr.rkey()
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaPtr<T, D> for SimpleProcRemotePtrMut<T, D> {
    unsafe fn offset(self, count: isize) -> Self {
        Self {
            proc: self.proc,
            rptr: unsafe { self.rptr.offset(count) },
        }
    }
}

#[derive_where(Debug, Default)]
pub struct SimpleLocalAttach<T: RmaPointable, D: RmaDevice + ?Sized> {
    ptr: UndropPtr<T>,
    lkey: D::LocalKey,
    rkey: D::RemoteKey,
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> SimpleLocalAttach<T, D> {
    pub unsafe fn reset(&mut self) {
        unsafe {
            self.ptr.reset();
        }
    }
}

impl<T: RmaPointable, D: RmaDevice + ?Sized> RmaLocalAttach<T, D> for SimpleLocalAttach<T, D> {
    unsafe fn new(ptr: *mut T, lkey: D::LocalKey, rkey: D::RemoteKey) -> Self {
        Self {
            ptr: UndropPtr::new(ptr),
            lkey,
            rkey,
        }
    }
    fn cast<U: RmaPointable>(mut self) -> D::LocalAttach<U> {
        let r = unsafe { D::LocalAttach::new(self.ptr.get().cast::<U>(), self.lkey, self.rkey) };
        unsafe {
            self.reset();
        }
        r
    }
    fn lptr(&self) -> D::LocalPtr<T> {
        D::LocalPtr::new(self.ptr.get(), self.lkey)
    }
    fn lptr_mut(&self) -> D::LocalPtrMut<T> {
        D::LocalPtrMut::new(self.ptr.get(), self.lkey)
    }
    fn rptr(&self) -> <D as RmaDevice>::RemotePtr<T> {
        D::RemotePtr::new(self.ptr.get(), self.rkey)
    }
    fn rptr_mut(&self) -> <D as RmaDevice>::RemotePtrMut<T> {
        D::RemotePtrMut::new(self.ptr.get(), self.rkey)
    }
}
