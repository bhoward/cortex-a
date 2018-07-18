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

//! Current Exception Level

pub use register::cpu::RegisterReadOnly;

register_bitfields! {u32,
    CURRENTEL [
        /// Current exception level. Possible values:
        /// 00 EL0 (but you can't read CurrentEL at EL0...)
        /// 01 EL1
        /// 10 EL2
        /// 11 EL3
        EL              OFFSET(2)  NUMBITS(2) []
    ]
}

pub struct Reg;

impl RegisterReadOnly<u32, CURRENTEL::Register> for Reg {
    sys_coproc_read_raw!(u32, "CurrentEL");
}

#[allow(non_upper_case_globals)]
pub static CurrentEL: Reg = Reg {};
