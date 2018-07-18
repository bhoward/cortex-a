//! Processor core registers

#[macro_use]
mod macros;

pub mod cntfrq_el0;
pub mod cnthctl_el2;
pub mod cntp_ctl_el0;
pub mod cntp_tval_el0;
pub mod cntvoff_el2;
pub mod currentel;
pub mod hcr_el2;
pub mod mpidr_el1;
pub mod sp_el1;
pub mod sp;
