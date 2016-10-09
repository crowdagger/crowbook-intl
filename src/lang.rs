// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use error::{Error,Result};
use common::find_string;

use std::collections::HashMap;

/// Struct used to store localization information for a language.
pub struct Lang {
    /// The lang code
    pub lang: String,
    /// The content of localization
    pub content: HashMap<String, String>,
}

impl Lang {
    /// Create a new empty Lang with no content
    pub fn new<S>(lang: S) -> Lang
        where S: Into<String> {
        Lang {
            lang: lang.into(),
            content: HashMap::new(),
        }
    }

    /// Create a new Lang from a string
    ///
    /// This string should vaguely follow .po/.mo files: it can contain
    /// comments starting by a `#`, and an entry should be of the form:
    ///
    /// ```text, no_run
    /// msgid "Initial string"
    /// msgstr "Translated string"
    /// ```
    pub fn new_from_str<S>(lang: S, s: &str) -> Result<Lang>
        where S: Into<String> {
        let mut lang = Self::new(lang);
        let lines:Vec<_> = s.lines()
            .map(|s| s.trim())
            .collect();
        let mut i = 0;
        while i < lines.len() {
            if lines[i].is_empty() || lines[i].starts_with("#") {
                // empty line or comment, ignore
                i += 1;
                continue;
            }
            if let Some(begin) = lines[i].find("msgid") {
                let end = begin + "msgid".len();
                let mut s = &lines[i][end..];
                let mut key = String::new();
                loop {
                    key.push_str(&try!(find_string(s.as_bytes()).map_err(|e| {
                        Error::parse(format!("initializing lang '{}' at line {}, could not parse {} as a String: {}",
                                             &lang.lang, i, s, e))
                    })));
                    if i >= lines.len() - 1 || lines[i+1].starts_with("msgstr") {
                            break;
                    } else if lines[i+1].starts_with('"') {
                        i = i + 1;
                        s = lines[i];
                    } else {
                        return Err(Error::parse(format!("initializing lang '{}' at line {}, found 'msgid' without matching 'msgstr on next line",
                                                        &lang.lang,
                                                        i)));
                    }
                }
                i += 1;
                if let Some(begin) = lines[i].find("msgstr") {
                    let end = begin + "msgstr".len();
                    let mut s = &lines[i][end..];
                    let mut value = String::new();
                    loop {
                        value.push_str(&try!(find_string(s.as_bytes()).map_err(|e| {
                        Error::parse(format!("initializing lang '{}' at line {}, could not parse {} as a String: {}",
                                             &lang.lang,
                                             i,
                                             s,
                                             e))
                        })));
                        if i >= lines.len() - 1 || lines[i+1].is_empty() {
                            break;
                        } else {
                            i = i + 1;
                            s = lines[i];
                        }
                    }
                    if !key.is_empty() && !value.is_empty() {
                        lang.insert(key, value);
                    }
                } else {
                    unreachable!()
                }
                i += 1;
            } else {
                return Err(Error::parse(format!("initializing lang '{}' at line {}, unexected input: '{}'",
                                                &lang.lang, i, lines[i])));
            }
        }
        Ok(lang)
    }

    /// Insert a (key, value) pair in the HashMap containing localization strings
    ///
    /// # Arguments:
    /// * `key`: the string in default language
    /// * `value`: the translation in this language
    pub fn insert<S1, S2>(&mut self, key: S1, value: S2)
        where S1: Into<String>,
              S2: Into<String> {
        self.content.insert(key.into(), value.into());
    }
}



#[test]
fn lang_new_valid_1() {
    let s = r#"
# Some comment
msgid "Some string"
msgstr "Une chaîne"

# Other comment
msgid "Other string"
msgstr "Autre chaîne"
"#;
    Lang::new_from_str("fr", s).unwrap();
}


#[test]
fn lang_new_invalid_1() {
    let s = r#"
msgstr "Msgstr first"
msgid "Some string"
msgstr "Une chaîne"

# Other comment
msgid "Other string"
msgstr "Autre chaîne"
"#;
    let lang = Lang::new_from_str("fr", s);
    assert!(lang.is_err());
}

#[test]
fn lang_new_invalid_2() {
    let s = r#"
msgid "Some string"
msgid "Two consecutive msgid without msgstr"

# Other comment
msgid "Other string"
msgstr "Autre chaîne"
"#;
    let lang = Lang::new_from_str("fr", s);
    assert!(lang.is_err());
}

#[test]
fn lang_multiline_1() {
    let s = r#"
msgid "foo"
msgstr ""
"foo"
"bar"
"#;
    let lang = Lang::new_from_str("fr", s).unwrap();
    assert_eq!(lang.content.get("foo").unwrap(), "foobar");
}

#[test]
fn lang_multiline_2() {
    let s = r#"
msgid "foo"
"bar"
msgstr ""
"foo"
"bar"
"#;
    let lang = Lang::new_from_str("fr", s).unwrap();
    assert_eq!(lang.content.get("foobar").unwrap(), "foobar");
}

#[test]
fn lang_empty() {
    let s = r#"
msgid "foo"
msgstr ""

msgid ""
msgstr "bar"
"#;
    let lang = Lang::new_from_str("fr", s).unwrap();
    assert_eq!(lang.content.len(), 0);
}
