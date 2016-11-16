/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod ipdl;
pub mod ast;
pub mod uncommenter;

use std::io::prelude::*;
use std::fs::File;
use ast::ParserState;

use std::env;

fn main() {
    let f_name = env::args().nth(1).unwrap();
    let mut f = File::open(f_name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s = uncommenter::uncomment(&s);

    println!("Output: {:?}", ipdl::parse_TranslationUnit(&ParserState::new(), &s).unwrap());
}
