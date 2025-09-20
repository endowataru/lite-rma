use std::sync::Arc;

use lite_rma::{
    coll::traits::CollDevice,
    dev::mpi::{error::MpiError, mpi_com::MpiCom},
    rma::{RmaDevice, RmaLocalPtrMut},
    structs::alltoall_mem::AlltoallMem,
    traits::ComBaseDevice,
    ult::sched::{OsSched, Sched},
    util::ptr::SendPtr,
};

fn main() {
    let sched = Arc::new(OsSched {});
    let _: Result<(), MpiError> = sched.block_on(async {
        let com = Arc::new(MpiCom::coll_new(sched.clone()).await?);
        let mem = AlltoallMem::<_, usize>::coll_new(&com, 1).await?;
        com.barrier().await?;
        let num_procs = com.num_procs();
        let this_proc = com.this_proc_id();
        let buf: usize = (this_proc + 1) * 100;
        let dest_proc = (this_proc + 1) % num_procs;
        unsafe {
            com.buf_write(
                SendPtr::new(&buf as *const usize),
                mem.prptr(dest_proc, 0),
                1,
            )
        }
        .await?;
        com.barrier().await?;
        let src_proc = (this_proc + num_procs - 1) % num_procs;
        assert_eq!(unsafe { *mem.lptr(0).ptr() }, (src_proc + 1) * 100);
        Ok(())
    });
}
