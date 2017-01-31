// Copyright 2016  Jonas mg
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::error;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AsciiError {
    ControlChar(usize),
    NonAscii(char),
}

impl error::Error for AsciiError {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            AsciiError::ControlChar(_) => "contain ASCII control character",
            AsciiError::NonAscii(_) => "contain non US-ASCII character",
        }
    }
}

impl fmt::Display for AsciiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AsciiError::ControlChar(pos) => write!(f, "{} at position {}", self.description(), pos),
            AsciiError::NonAscii(ch) => write!(f, "{} ({})", self.description(), ch),
        }
    }
}
