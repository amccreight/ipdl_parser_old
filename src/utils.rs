/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path::{Path, PathBuf};

pub struct IncludeResolver {
    include_dirs: Vec<PathBuf>,
}

impl IncludeResolver {
    pub fn new(include_dirs: &Vec<PathBuf>) -> IncludeResolver {
        IncludeResolver { include_dirs: include_dirs }
    }

    pub fn resolve(&self, file_path: &Path) -> Option<PathBuf> {
        // XXX The Python parser also checks '' for some reason.
        for d in &self.include_dirs {
            let mut p = d.clone();
            p.push(file_path);

            if p.exists() {
                if let Ok(pb) = p.canonicalize() {
                    return Some(pb)
                }
            }
        }

        return None
    }
}
