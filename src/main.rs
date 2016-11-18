/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod ast;
pub mod ipdl;
pub mod parser;
pub mod uncommenter;

use std::path::PathBuf;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Expected two arguments, an include path and a file name.");
    }

    let include_path = PathBuf::from(&args[1]);
    let file_name = &args[2];
    println!("Output: {:?}", parser::parse(&vec![include_path], file_name));
}
