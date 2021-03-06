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

//! Architectural Feature Access Control register - EL2

pub use register::cpu::RegisterReadWrite;

register_bitfields! {u32,
    CPACR_EL1 [
        /// TODO write more here
        FPEN           OFFSET(20)  NUMBITS(2) []
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, CPACR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "CPACR_EL1");
    sys_coproc_write_raw!(u32, "CPACR_EL1");
}

pub static CPACR_EL1: Reg = Reg {};
