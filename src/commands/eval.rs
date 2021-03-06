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

use super::{Command, Response};
use super::flag::Flag;
use xml::reader::XmlEvent;

response!(struct EvalResponse {});

// TODO: Test this
command!("stdin", struct Eval {
    data_page: Option<u32>: 'p',
    stack_depth: Option<u32>: 'd',
    data: [u8]: '-'
}, EvalResponse, |i: &Eval, xml: XmlEvent| {
    EvalResponse{}
});


response!(struct ExprResponse {});

// TODO: Test this
command!("stdin", struct Expr {
    data_page: Option<u32>: 'p',
    stack_depth: Option<u32>: 'd',
    data: [u8]: '-'
}, ExprResponse, |i: &Expr, xml: XmlEvent| {
    ExprResponse{}
});


response!(struct ExecResponse {});

// TODO: Test this
command!("stdin", struct Exec {
    data: [u8]: '-'
}, ExecResponse, |i: &Exec, xml: XmlEvent| {
    ExecResponse{}
});
