#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct UnsafeSend<T: Copy>(T);

unsafe impl<T: Copy> Send for UnsafeSend<T> {}
unsafe impl<T: Copy> Sync for UnsafeSend<T> {}

impl<T: Copy> UnsafeSend<T> {
    pub fn new(arg: T) -> Self {
        UnsafeSend(arg)
    }

    pub fn get(&self) -> T {
        self.0
    }
}

impl<T: Copy> From<T> for UnsafeSend<T> {
    fn from(arg: T) -> Self {
        UnsafeSend(arg)
    }
}

impl<T: Copy> AsRef<T> for UnsafeSend<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
impl<T: Copy> AsMut<T> for UnsafeSend<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

pub type SendRequest = UnsafeSend<mpi::ffi::MPI_Request>;
pub type SendComm = UnsafeSend<mpi::ffi::MPI_Comm>;
pub type SendDatatype = UnsafeSend<mpi::ffi::MPI_Datatype>;
pub type SendWin = UnsafeSend<mpi::ffi::MPI_Win>;
pub type SendOp = UnsafeSend<mpi::ffi::MPI_Op>;
pub type SendStatus = UnsafeSend<mpi::ffi::MPI_Status>;
