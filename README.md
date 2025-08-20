# LiteRMA: A communication library for HPC systems in Rust

This library is designed to simplify using Remote Memory Access (RMA) in Rust
and implement RMA-based libraries easily.
RMA is a set of the interface in High-Performance Computing (HPC) systems
and is typically implemented on a hardware feature called Remote Direct Memory Access (RDMA).
The aim of this library is to exploit the performance of RDMA from Rust-based libraries.

The current implementation is based on MPI-3 RMA, which is not efficient as expected,
but my intention is to expand the underlying layer to libfabric or UCX,
which can effectively access the HPC interconnect hardware.

This project is very experimental. The APIs may vary without proper notices.
Versioning of this project may not follow semantic versioning.
