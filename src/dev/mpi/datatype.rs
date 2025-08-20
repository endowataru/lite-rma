/// Get MPI_Datatype based on the size of T.
/// It panics when the size is none of 1, 2, 4, and 8.
pub fn get_datatype_from_size<T>() -> mpi::ffi::MPI_Datatype {
    unsafe {
        match std::mem::size_of::<T>() {
            1 => mpi::ffi::RSMPI_INT8_T,
            2 => mpi::ffi::RSMPI_INT16_T,
            4 => mpi::ffi::RSMPI_INT32_T,
            8 => mpi::ffi::RSMPI_INT64_T,
            _ => panic!(),
        }
    }
}

pub fn get_datatype<T: 'static>() -> mpi::ffi::MPI_Datatype {
    use std::any::TypeId;
    unsafe {
        match TypeId::of::<T>() {
            t if t == TypeId::of::<i8>() => mpi::ffi::RSMPI_INT8_T,
            t if t == TypeId::of::<i16>() => mpi::ffi::RSMPI_INT16_T,
            t if t == TypeId::of::<i32>() => mpi::ffi::RSMPI_INT32_T,
            t if t == TypeId::of::<i64>() => mpi::ffi::RSMPI_INT64_T,
            t if t == TypeId::of::<u8>() => mpi::ffi::RSMPI_UINT8_T,
            t if t == TypeId::of::<u16>() => mpi::ffi::RSMPI_UINT16_T,
            t if t == TypeId::of::<u32>() => mpi::ffi::RSMPI_UINT32_T,
            t if t == TypeId::of::<u64>() => mpi::ffi::RSMPI_UINT64_T,
            t if t == TypeId::of::<f32>() => mpi::ffi::RSMPI_FLOAT,
            t if t == TypeId::of::<f64>() => mpi::ffi::RSMPI_DOUBLE,
            _ => panic!("Unsupported type for MPI datatype"),
        }
    }
}
