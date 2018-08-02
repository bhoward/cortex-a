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

//! Saved Program Status register - EL3

pub use register::cpu::RegisterReadWrite;

register_bitfields! {u32,
    SPSR_EL3 []
}

pub struct Reg;

impl RegisterReadWrite<u32, SPSR_EL3::Register> for Reg {
    sys_coproc_read_raw!(u32, "SPSR_EL3");
    sys_coproc_write_raw!(u32, "SPSR_EL3");
}

pub static SPSR_EL3: Reg = Reg {};
