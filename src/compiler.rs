/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use parser;
use std::path::PathBuf;
use type_check;

pub fn compile(include_dirs: &Vec<PathBuf>, file_names: Vec<PathBuf>) -> Result<(), String> {
    let tus = parser::parse_with_errors(&include_dirs, file_names)?;
    type_check::check(&tus)?;
    Ok(())
}
