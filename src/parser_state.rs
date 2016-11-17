/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::Cell;
use ast::Direction;

pub struct ParserState {
    pub direction: Cell<Option<Direction>>,
}

impl ParserState {
    pub fn new() -> ParserState {
        ParserState { direction: Cell::new(None) }
    }
}
