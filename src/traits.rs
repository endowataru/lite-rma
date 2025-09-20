use crate::{ProcInt, coll::traits::CollDevice, rma::RmaDevice, ult::sched::Sched};

pub trait ComBaseDevice: Sync {
    type Error: std::error::Error + Send;

    /// Get the ID of this process.
    fn this_proc_id(&self) -> ProcInt;

    /// Get the number of processes.
    fn num_procs(&self) -> ProcInt;

    type Sched: Sched;
    fn sched(&self) -> &Self::Sched;
}

pub trait ComDevice: RmaDevice + CollDevice { }
