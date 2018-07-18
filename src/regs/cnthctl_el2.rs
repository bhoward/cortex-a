/*
 * Copyright (c) 2018 by the author(s)
 *
 * =============================================================================
 *
 * Licensed under either of
 *   - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
 *   - MIT License (http://opensource.org/licenses/MIT)
 * at your option.
 *
 * =============================================================================
 *
 * Author(s):
 *   - Andre Richter <andre.o.richter@gmail.com>
 *   - Brian Howard <bhoward@depauw.edu>
 */

//! Counter-timer Hypervisor Control register - EL2

pub use register::cpu::RegisterReadWrite;

register_bitfields! {u32,
    CNTHCTL_EL2 [
        /// NOTE: The following fields assume EL2 is running a hypervisor
        /// (HCR_EL2.E2H is 0); when EL2 is running a host operating system,
        /// the fields are different.
        
        /// Traps non-secure EL0/EL1 accesses to the physical counter register
        /// to EL2. Permitted values are:
        ///
        /// 0 Non-secure accesses to CNTPCT_EL0 are trapped to EL2.
        /// 1 Instructions are not trapped.
        EL1PCTEN        OFFSET(0)  NUMBITS(1) [],

        /// Traps non-secure EL0/EL1 accesses to the physical timer registers
        /// to EL2. Permitted values are:
        ///
        /// 0 Non-secure accesses to CNTP_CTL_EL0, CNTP_CVAL_EL0, and CNTP_TVAL_EL0
        ///   are trapped to EL2.
        /// 1 Instructions are not trapped.
        EL1PCEN         OFFSET(1)  NUMBITS(1) [],

        /// Enables the generation of an event stream from the counter register
        /// CNTPCT_EL0:
        ///
        /// 0 Disables the event stream.
        /// 1 Enables the event stream.
        EVNTEN          OFFSET(2)  NUMBITS(1) [],

        /// Controls which transition of the counter register CNTPCT_EL0 trigger
        /// bit, defined by EVNTI, generates an event when the event stream is
        /// enabled:
        ///
        /// 0 A 0 to 1 transition of the trigger bit triggers an event.
        /// 1 A 1 to 0 transition of the trigger bit triggers an event.
        EVNTDIR         OFFSET(3)  NUMBITS(1) [],

        /// Selects which bit (0 to 15) of the counter register CNTPCT_EL0 is the
        /// trigger for the event stream generated from that counter, when that
        /// stream is enabled.
        EVNTI           OFFSET(4)  NUMBITS(4) []
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, CNTHCTL_EL2::Register> for Reg {
    sys_coproc_read_raw!(u32, "CNTHCTL_EL2");
    sys_coproc_write_raw!(u32, "CNTHCTL_EL2");
}

pub static CNTHCTL_EL2: Reg = Reg {};
