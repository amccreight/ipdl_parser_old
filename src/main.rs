/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate regex;

pub mod ipdl;
pub mod ast;
pub mod uncommenter;

use ast::ParserState;

fn main() {
    println!("Output: {:?}", ipdl::parse_IncludeStmt(&ParserState::new(), "include      Whatever").unwrap());
    println!("Output: {:?}", ipdl::parse_CxxIncludeStmt(&ParserState::new(), "include   \"hello.h\"").unwrap());
}
