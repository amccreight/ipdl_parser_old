/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use parser;
use std::path::PathBuf;
use type_check;

pub fn compile(include_dirs: &Vec<PathBuf>, file_names: Vec<PathBuf>) -> bool {
    let maybe_tus = parser::parse(&include_dirs, file_names);

    if maybe_tus.is_none() {
        println!("Specification could not be parsed.");
        return false;
    }

    let tus = maybe_tus.unwrap();
    if let Err(err) = type_check::check(&tus) {
        println!("Error(s) during type checking.\n{}", err);
        return false;
    }

    true
}
