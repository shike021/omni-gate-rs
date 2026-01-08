//! OmniGate library
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

pub mod protos {
    pub mod helloworld {
        include!(concat!(env!("OUT_DIR"), "/helloworld.rs"));
    }
    pub mod user {
        include!(concat!(env!("OUT_DIR"), "/user.rs"));
    }
}
