use std::sync::Arc;

use crate::{
    ProcInt,
    dev::mpi::{
        communicator::Communicator, direct_communicator::DirectCommunicator,
        direct_mpi_sched::DirectMpiSched, error::MpiError, mpi_coll::MpiCollDevice,
    },
    traits::ComBaseDevice,
    ult::sched::Sched,
};

pub type DirectMpiCommunicator<S> = DirectCommunicator<DirectMpiSched<S>>;

pub struct DirectMpiCollDevice<S: Sched> {
    comm: Arc<DirectMpiCommunicator<S>>,
}

impl<S: Sched> DirectMpiCollDevice<S> {
    pub fn new(comm: Arc<DirectMpiCommunicator<S>>) -> Self {
        Self { comm }
    }
}

impl<S: Sched> MpiCollDevice for DirectMpiCollDevice<S> {
    type Communicator = DirectMpiCommunicator<S>;
    fn comm(&self) -> &Self::Communicator {
        self.comm.as_ref()
    }
}

impl<S: Sched> ComBaseDevice for DirectMpiCollDevice<S> {
    fn this_proc_id(&self) -> ProcInt {
        self.comm().rank() as ProcInt
    }

    fn num_procs(&self) -> ProcInt {
        self.comm().size() as ProcInt
    }

    type Sched = S;
    fn sched(&self) -> &Self::Sched {
        self.comm().sched().sched()
    }

    type Error = MpiError;
}
