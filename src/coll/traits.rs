// TODO: Do not depend on MPI implementation at this layer
use mpi::traits::Equivalence;

use crate::traits::ComBaseDevice;

// TODO: remove dependency on MPI in this layer
pub type SystemOperation = mpi::collective::SystemOperation;

pub trait CollDevice: ComBaseDevice {
    type Error: std::error::Error + Send;

    fn barrier(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn allgather<T>(
        &self,
        send_buf: &[T],
        recv_buf: &mut [T],
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        T: Copy + Send + Sync;

    fn allreduce<T>(
        &self,
        send_buf: &[T],
        recv_buf: &mut [T],
        op: SystemOperation,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        T: Copy + Send + Sync + Equivalence;
}
