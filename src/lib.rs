/*
 * Copyright (c) the dbgp contributors. All rights reserved.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation. This file is also subject
 * to the Linking exception provided in the LICENSE file that
 * accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 */

// TODO: Make this work
#![deny(
    //missing_docs,
    missing_debug_implementations,
    warnings
)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

//! This library implements the dbgp protocol

#[macro_use]
extern crate error_chain;
extern crate base64;
extern crate url;
extern crate url_serde;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[macro_use]
mod macros;
pub mod escape;
pub mod commands;
pub mod types;
pub mod packets;
mod helpers;

use std::io;
pub use types::*;

/// Represents a session with a debugger
#[derive(Debug, Clone, PartialEq)]
pub struct Session<RW: io::Read + io::Write> {
    /// Communication channel
    channel: RW,

    /// Represents the debugger status
    status: SessionStatus,

    session_type: SessionType,
}

impl<RW: io::Read + io::Write> Session<RW> {
    /// Creates a new session with the default parameters
    pub fn new(channel: RW, session_type: SessionType) -> Session<RW> {
        Session {
            channel,
            session_type,
            status: SessionStatus::Starting,
        }
    }

    //pub fn try_parse(&mut self) -> Result<Command> {
    //}
}
