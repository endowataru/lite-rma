use std::error::Error;

use crate::{ProcInt, coll::traits::CollDevice, rma::RmaDevice, ult::sched::Sched};

pub trait ComBaseDevice: Sync {
    /// Get the ID of this process.
    fn this_proc_id(&self) -> ProcInt;

    /// Get the number of processes.
    fn num_procs(&self) -> ProcInt;
}

pub trait ComDevice {
    type RmaDevice: RmaDevice;
    fn rma(&self) -> &Self::RmaDevice;

    type CollDevice: CollDevice;
    fn coll(&self) -> &Self::CollDevice;

    type Sched: Sched;
    fn sched(&self) -> &Self::Sched;

    type Error: Error
        + From<<Self::RmaDevice as RmaDevice>::Error>
        + From<<Self::CollDevice as CollDevice>::Error>;
}
