/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate getopts;

pub mod ast;
pub mod ipdl;
pub mod parser;
pub mod uncommenter;

use getopts::Options;
use std::path::PathBuf;
use std::env;

fn get_options_parser() -> Options {
    let mut opts = Options::new();
    opts.optmulti("I", "include",
                  "Additional directory to search for included protocol specifications",
                  "DIR");
    opts.optopt("d", "outheaders-dir",
                "Directory into which C++ headers will be generated. \
                 A protocol Foo in the namespace bar will cause the headers \
                 dir/bar/Foo.h, dir/bar/FooParent.h, and dir/bar/FooParent.h \
                 to be generated",
                "HDR_DIR");
    opts.optopt("o", "outcpp-dir",
                "Directory into which C++ sources will be generated \
                A protocol Foo in the namespace bar will cause the sources \
                cppdir/FooParent.cpp, cppdir/FooChild.cpp \
                to be generated",
                "CPP_DIR");
    opts
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let opts = get_options_parser();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };

    if matches.free.is_empty() {
        panic!("Expected at least one IPDL file to be specified.");
    }

    let mut include_dirs = Vec::new();
    for i in matches.opt_strs("I") {
        include_dirs.push(PathBuf::from(i))
    }

    let mut file_names = Vec::new();
    for f in matches.free {
        file_names.push(PathBuf::from(f));
    }

    println!("Output: {:?}", parser::parse(&include_dirs, file_names).len());
}
