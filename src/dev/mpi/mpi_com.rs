use std::sync::Arc;

use mpi::{environment::Universe, raw::AsRaw};

use crate::{
    dev::mpi::{
        direct_communicator::DirectCommunicator, direct_mpi_coll::DirectMpiCollDevice,
        direct_mpi_rma::DirectMpiRmaDevice, direct_mpi_sched::DirectMpiSched, error::MpiError,
        mpi_rma::MpiRmaDevice, send::SendComm, window::Window,
    },
    traits::ComDevice,
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

impl<S: Sched> ComDevice for MpiCom<S> {
    type RmaDevice = DirectMpiRmaDevice<S>;
    fn rma(&self) -> &Self::RmaDevice {
        &self.rma
    }

    type CollDevice = DirectMpiCollDevice<S>;
    fn coll(&self) -> &Self::CollDevice {
        &self.coll
    }

    type Sched = DirectMpiSched<S>;
    fn sched(&self) -> &Self::Sched {
        self.rma().window().sched()
    }

    type Error = MpiError;
}
