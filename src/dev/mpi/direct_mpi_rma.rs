use std::{future::Future, sync::Arc};

use crate::{
    ProcInt,
    dev::mpi::{
        communicator::Communicator, direct_communicator::DirectCommunicator,
        direct_mpi_sched::DirectMpiSched, direct_window::DirectWindow, error::MpiError,
        mpi_rma::MpiRmaDevice, window::Window,
    },
    traits::ComBaseDevice,
    ult::sched::Sched,
};

type Comm<S> = DirectCommunicator<DirectMpiSched<S>>;
type Win<S> = DirectWindow<Comm<S>>;

pub struct DirectMpiRmaDevice<S: Sched> {
    win: Box<Win<S>>,
}

impl<S: Sched> DirectMpiRmaDevice<S> {
    pub fn coll_new(comm: Arc<Comm<S>>) -> impl Future<Output = Result<Self, MpiError>> + Send {
        async move {
            let win = unsafe { Win::create_dynamic(comm) }.await?;
            unsafe { win.lock_all(0) }.await?;
            Ok(Self { win })
        }
    }
}

impl<S: Sched> Drop for DirectMpiRmaDevice<S> {
    fn drop(&mut self) {
        self.win
            .sched()
            .block_on(unsafe { self.win.unlock_all() })
            .unwrap()
    }
}

impl<S: Sched> MpiRmaDevice for DirectMpiRmaDevice<S> {
    type Window = Win<S>;
    fn window(&self) -> &Self::Window {
        self.win.as_ref()
    }
}

impl<S: Sched> ComBaseDevice for DirectMpiRmaDevice<S> {
    fn this_proc_id(&self) -> ProcInt {
        self.window().comm().rank() as ProcInt
    }

    fn num_procs(&self) -> ProcInt {
        self.window().comm().size() as ProcInt
    }
}
