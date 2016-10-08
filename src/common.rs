// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use error::{Error, Result};

/// Find the next string, delimited by quotes `"` (which are not returned),
/// and not stopping at escape quotes `\"`
pub fn find_string(bytes: &[u8]) -> Result<String> {
    let mut begin = None;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => if begin.is_some() {
                if bytes[i-1] != b'\\' {
                    break
                }
            } else {
                if i + 1 >= bytes.len() {
                    return Err(Error::new(""));
                }
                begin = Some(i + 1);
            },
            _ => (),
        }
        i += 1;
    }
    let begin = if let Some(begin) = begin {
        begin
    } else {
        return Err(Error::new(""));
    };
    Ok(String::from_utf8(bytes[begin..i].to_vec()).unwrap())
}
