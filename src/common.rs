// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use error::{Error, Result};

use std::borrow::Cow;

/// Escape some special characters that would cause trouble
pub fn escape_string<'a>(s: &'a str) -> Cow<'a, str> {
    if s.contains('\n') {
        let res = s.replace('\n', r"\n");
        Cow::Owned(res)
    } else {
        Cow::Borrowed(s)
    }
}

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


#[test]
fn find_string_1() {
    let s = r#"
"Test"
"#;
    let expected = "Test";
    assert_eq!(&find_string(s.as_bytes()).unwrap(), expected);
}

#[test]
fn find_string_2() {
    let s = r#"
"A \"test\"..."
"#;
    let expected = r#"A \"test\"..."#;
    assert_eq!(&find_string(s.as_bytes()).unwrap(), expected);
}

#[test]
fn escape_string_1() {
    let s = r#"foo
bar"#;
    let expected = "foo\\nbar";
    assert_eq!(&escape_string(s), expected);
}
