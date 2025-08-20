use std::sync::Arc;

use crate::{
    ProcInt,
    dev::mpi::{
        communicator::Communicator, direct_communicator::DirectCommunicator,
        direct_mpi_sched::DirectMpiSched, mpi_coll::MpiCollDevice,
    },
    traits::ComBaseDevice,
    ult::sched::Sched,
};

type Comm<S> = DirectCommunicator<DirectMpiSched<S>>;

pub struct DirectMpiCollDevice<S: Sched> {
    comm: Arc<Comm<S>>,
}

impl<S: Sched> DirectMpiCollDevice<S> {
    pub fn new(comm: Arc<Comm<S>>) -> Self {
        Self { comm }
    }
}

impl<S: Sched> MpiCollDevice for DirectMpiCollDevice<S> {
    type Communicator = Comm<S>;
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
}
