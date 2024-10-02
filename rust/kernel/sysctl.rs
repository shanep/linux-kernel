// SPDX-License-Identifier: GPL-2.0

//! Sysctl definitions
//!
//! C header: [`include/linux/sysctl.h`](srctree/include/linux/sysctl.h).

///Gets a number for stuff
pub const fn get_number() -> i32 {
    2
}

pub const fn get_second() -> i32 {
    3
}
