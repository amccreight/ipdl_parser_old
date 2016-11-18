/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod ast;
pub mod ipdl;
pub mod parser_state;
pub mod uncommenter;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use parser_state::ParserState;

use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Expected two arguments, an include path and a file name.");
    }

    let include_path = PathBuf::from(&args[1]);
    let mut f = File::open(&args[2]).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s = uncommenter::uncomment(&s);

    let parser_state = ParserState::new(vec![include_path]);
    println!("Output: {:?}", ipdl::parse_TranslationUnit(&parser_state, &s).unwrap());
}
