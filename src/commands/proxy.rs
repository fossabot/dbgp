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

response!(struct ProxyInitResponse {});

command!("proxyinit", struct ProxyInit {
    port:        u32:          'a',
    ide_key:     String:       'k',
    multi_debug: Option<bool>: 'm'
}, ProxyInitResponse, |i: &ProxyInit, xml: XmlEvent| {
    ProxyInitResponse{}
});

response!(struct ProxyStopResponse {});

command!("proxystop", struct ProxyStop {
    ide_key: String: 'k'
}, ProxyStopResponse, |i: &ProxyStop, xml: XmlEvent| {
    ProxyStopResponse{}
});
