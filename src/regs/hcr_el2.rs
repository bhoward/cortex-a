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

//! Hypervisor Configuration register - EL2

pub use register::cpu::RegisterReadWrite;

register_bitfields! {u64,
    HCR_EL2 [
        /// TODO this is incomplete -- there are 36 more fields...
        
        /// Register Width -- determines execution state (AArch32/64) for
        /// lower exception levels. Permitted values are:
        ///
        /// 0 Lower levels (EL1 and EL0) are AArch32.
        /// 1 EL1 is AArch64. Execution state for EL0 is determined by
        ///   PSTATE.nRW.
        RW              OFFSET(31)  NUMBITS(1) [],

        /// Set/Way Invalidation Override. Causes non-secure EL1 data cache
        /// invalidation to also perform a clean. Permitted values are:
        ///
        /// 0 No effect on data cache invalidate by set/way instructions.
        /// 1 Data cache invalidate by set/way instructions perform a
        ///   data cache clean and invalidate by set/way.
        SWIO            OFFSET(1)  NUMBITS(1) []
    ]
}

pub struct Reg;

impl RegisterReadWrite<u64, HCR_EL2::Register> for Reg {
    sys_coproc_read_raw!(u64, "HCR_EL2");
    sys_coproc_write_raw!(u64, "HCR_EL2");
}

pub static HCR_EL2: Reg = Reg {};
