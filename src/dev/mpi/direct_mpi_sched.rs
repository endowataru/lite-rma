use crate::{
    dev::mpi::{error::MpiError, mpi_sched::MpiSched, send::SendRequest},
    ult::sched::Sched,
    util::ptr::SendPtrMut,
};
use std::{ffi::c_int, sync::Arc};

pub struct DirectMpiSched<S: Sched> {
    sched: Arc<S>,
}

impl<S: Sched> DirectMpiSched<S> {
    pub fn new(sched: Arc<S>) -> Self {
        Self { sched }
    }
}

impl<S: Sched> MpiSched for DirectMpiSched<S> {
    unsafe fn test(
        &self,
        req: SendRequest,
        status: SendPtrMut<mpi::ffi::MPI_Status>,
    ) -> impl Future<Output = Result<c_int, MpiError>> + Send {
        async move {
            let mut req = req;
            let mut flag: c_int = 0;
            MpiError::check(unsafe {
                mpi::ffi::MPI_Test(
                    req.as_mut() as *mut mpi::ffi::MPI_Request,
                    &mut flag as *mut c_int,
                    status.get(),
                )
            })?;
            Ok(flag)
        }
    }

    unsafe fn wait(&self, req: SendRequest) -> impl Future<Output = Result<(), MpiError>> + Send {
        async move {
            loop {
                let flag = unsafe { self.test_status_ignore(req) }.await?;
                if flag != 0 {
                    return Ok(());
                }
                self.sched.yield_now().await;
            }
        }
    }
}

impl<S: Sched> Sched for DirectMpiSched<S> {
    fn yield_now(&self) -> impl std::future::Future<Output = ()> + Send {
        self.sched.yield_now()
    }

    fn block_on<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T> + Send,
        T: Send,
    {
        self.sched.block_on(future)
    }
}
