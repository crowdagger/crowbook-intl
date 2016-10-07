// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

/// Struct used to store localization information for a language.
pub struct Lang {
    /// The lang code
    pub lang: String,
    /// The content of localization
    pub content: HashMap<String, String>,
}

impl Lang {
    /// Create a new empty lang with no content
    pub fn new<S>(lang: S) -> Lang
        where S: Into<String> {
        Lang {
            lang: lang.into(),
            content: HashMap::new(),
        }
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
