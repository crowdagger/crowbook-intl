// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::error;
use std::result;
use std::fmt;

/// Internal ErrorType
#[derive(Debug, PartialEq)]
enum ErrorType {
    Default,
    Parse,
}

/// Result type (returned by most methods of this library)
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
/// Error type returned by methods of this library
pub struct Error {
    msg: String,
    variant: ErrorType
}

impl Error {
    /// Creates a new default error
    pub fn new<S: Into<String>>(msg: S) -> Error {
        Error {
            msg: msg.into(),
            variant: ErrorType::Default,
        }
    }

    /// Creates a new parse error
    pub fn parse<S: Into<String>>(msg: S) -> Error {
        Error {
            msg: msg.into(),
            variant: ErrorType::Parse,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.variant {
            ErrorType::Parse => write!(f, "Error parsing localization file: {}", self.msg),
            _ => write!(f, "{}", self.msg),
        }
    }
}

