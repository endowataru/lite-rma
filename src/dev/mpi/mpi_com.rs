use std::sync::Arc;

use mpi::{environment::Universe, raw::AsRaw};

use crate::{
    ProcInt,
    dev::mpi::{
        direct_communicator::DirectCommunicator,
        direct_mpi_coll::{DirectMpiCollDevice, DirectMpiCommunicator},
        direct_mpi_rma::{DirectMpiRmaDevice, DirectMpiWindow},
        direct_mpi_sched::DirectMpiSched,
        error::MpiError,
        mpi_coll::MpiCollDevice,
        mpi_rma::MpiRmaDevice,
        send::SendComm,
    },
    traits::{ComBaseDevice, ComDevice},
    ult::sched::Sched,
};

pub struct MpiCom<S: Sched> {
    rma: DirectMpiRmaDevice<S>,
    coll: DirectMpiCollDevice<S>,
    // Destruct Universe at last.
    _universe: Universe,
}

impl<S: Sched> MpiCom<S> {
    pub fn coll_new(sched: Arc<S>) -> impl Future<Output = Result<Self, MpiError>> + Send {
        async move {
            let (universe, _) = mpi::initialize_with_threading(mpi::Threading::Multiple).unwrap();
            let world = SendComm::new(universe.world().as_raw());

            let mpi_sched = Arc::new(DirectMpiSched::new(sched));
            let comm = Arc::new(DirectCommunicator::new(mpi_sched, world.get()));
            let coll = DirectMpiCollDevice::new(comm.clone());
            let rma = DirectMpiRmaDevice::coll_new(comm).await?;
            Ok(Self {
                rma,
                coll,
                _universe: universe,
            })
        }
    }
}

impl<S: Sched> ComBaseDevice for MpiCom<S> {
    type Error = MpiError;
    fn this_proc_id(&self) -> ProcInt {
        self.coll.num_procs()
    }
    fn num_procs(&self) -> ProcInt {
        self.coll.num_procs()
    }

    type Sched = DirectMpiSched<S>;
    fn sched(&self) -> &Self::Sched {
        todo!()
    }
}

impl<S: Sched> MpiRmaDevice for MpiCom<S> {
    type Window = DirectMpiWindow<S>;
    fn window(&self) -> &Self::Window {
        self.rma.window()
    }
}

impl<S: Sched> MpiCollDevice for MpiCom<S> {
    type Communicator = DirectMpiCommunicator<S>;
    fn comm(&self) -> &Self::Communicator {
        self.coll.comm()
    }
}

impl<S: Sched> ComDevice for MpiCom<S> {}
