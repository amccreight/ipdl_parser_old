/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path::PathBuf;
use parser;
use type_check;


pub fn compile(include_dirs: &Vec<PathBuf>, file_names: Vec<PathBuf>) -> bool {
    let maybe_tus = parser::parse(&include_dirs, file_names);

    if maybe_tus.is_none() {
        println!("Specification could not be parsed.");
        return false;
    }

    let tus = maybe_tus.unwrap();

    for (_, tu) in tus {
        if !type_check::check(&tu) {
            return false;
        }
    }

    return true;
}
