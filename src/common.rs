// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use error::{Error, Result};

use std::borrow::Cow;

use regex::Regex;

/// Escape some special characters that would cause trouble
///
/// The newline character
/// '\' followed by a newline
pub fn escape_string<'a, S:Into<Cow<'a, str>>>(s: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX:Regex = Regex::new(r#"\\\n\s*"#).unwrap();
    }

    let s = s.into();
    if s.contains('\n') || REGEX.is_match(&s) {
        let mut res = REGEX.replace_all(&s, "").into_owned();
        res = res.replace('\n', r"\n");
        Cow::Owned(res)
    } else {
        s
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

#[test]
fn escape_string_2() {
    let s = "foo\
             bar";
    let expected = "foobar";
    assert_eq!(&escape_string(s), expected);
}

#[test]
fn escape_string_3() {
    let s = r#"foo\
             bar"#;
    let expected = "foobar";
    assert_eq!(&escape_string(s), expected);
}


#[test]
fn escape_string_4() {
    let s = r#"foo\
             bar
baz"#;
    let expected = "foobar\\nbaz";
    assert_eq!(&escape_string(s), expected);
}
