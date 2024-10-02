// SPDX-License-Identifier: GPL-2.0

//! Research Module for Rust

use kernel::pr_cont;
use kernel::prelude::*;
use kernel::sysctl;


module! {
    type: RustSecurity,
    name: "rust_secuirty",
    author: "Shane K. Panter",
    description: "Rust Module for security research",
    license: "GPL",
}

struct RustSecurity;


impl kernel::Module for RustSecurity {
    fn init(_module: &'static ThisModule) -> Result<Self> {
	pr_info!("Rust Security Module (init)\n");
	let a = sysctl::get_number();

	pr_info!("Number was {}\n", a);
        Ok(RustSecurity)
    }
}

impl Drop for RustSecurity {
    fn drop(&mut self) {
        pr_info!("Rust Security Module (exit)\n");
    }
}
