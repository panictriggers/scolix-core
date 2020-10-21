// Copyright (C) 2020 PanicTriggers.
// All rights reserved.
//
// SPDX-License-Identifier: AGPL-3.0-only
//



// Don't build with warnings or unsafe code
// #![deny(warnings)]
#![deny(unsafe_code)]
#![allow(dead_code)]

mod consts;
mod db;
mod session;

fn main() {
    println!("Scolix Core {}", consts::BUILDVERSION);
}
