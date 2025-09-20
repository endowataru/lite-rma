use std::{mem::take, slice, sync::Arc};

use crate::rma::RmaProcRemotePtrMut;
use crate::{
    ProcInt,
    rma::{RmaLocalAttach, RmaPointable, RmaPtr},
    traits::ComDevice,
    ult::sched::Sched,
    util::ptr::SendPtrMut,
};

pub struct AlltoallMem<D: ComDevice, T: RmaPointable + Default + Clone> {
    dev: Arc<D>,
    buf: Vec<T>,
    la: D::LocalAttach<T>,
    ptrs: Vec<D::RemotePtrMut<T>>,
}

impl<D: ComDevice, T: RmaPointable + Default + Clone> AlltoallMem<D, T> {
    pub fn coll_new(dev: &Arc<D>, size: usize) -> impl Future<Output = Result<Self, D::Error>> {
        async move {
            let mut buf = vec![T::default(); size];
            let la = unsafe { dev.attach(SendPtrMut::new(buf.as_mut_ptr()), size) }.await?;
            let rptr = la.rptr_mut();
            let mut ptrs = vec![D::RemotePtrMut::<T>::default(); dev.num_procs()];
            dev.allgather(slice::from_ref(&rptr), ptrs.as_mut_slice())
                .await?;
            Ok(Self {
                dev: dev.clone(),
                buf,
                la,
                ptrs,
            })
        }
    }

    pub fn prptr(&self, proc: ProcInt, index: usize) -> D::ProcRemotePtrMut<T> {
        assert!(index < self.buf.len());
        unsafe { D::ProcRemotePtrMut::new(proc, self.ptrs[proc].add(index)) }
    }

    pub fn lptr(&self, index: usize) -> D::LocalPtrMut<T> {
        assert!(index < self.buf.len());
        unsafe { self.la.lptr_mut().add(index) }
    }
}

impl<D: ComDevice, T: RmaPointable + Default + Clone> Drop for AlltoallMem<D, T> {
    fn drop(&mut self) {
        self.dev
            .sched()
            .block_on(unsafe { self.dev.detach(take(&mut self.la)) })
            .unwrap()
    }
}
