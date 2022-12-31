// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::lang::Lang;
use crate::error::{Result, Error};
use crate::macrogen;
use crate::extractor::Extractor;

use std::fs::File;
use std::path::Path;
use std::io::Write;

/// Main struct for initiating localization for a project.
///
/// # Example
///
/// ```rust
/// use crowbook_intl::{Localizer, Extractor};
/// let fr = r#"
/// msgid "Hello, {}"
/// msgstr "Bonjour, {}"
/// "#;
/// let es = r#"
/// msgid "Hello, {}"
/// msgstr "Hola, {}"
/// "#;
/// let extractor = Extractor::new();
/// let mut localizer = Localizer::new(&extractor);
/// localizer.add_lang("fr", fr).unwrap();
/// localizer.add_lang("es", es).unwrap();
/// println!("{}", localizer.generate_macro_file());
/// ```
#[derive(Debug, Clone)]
pub struct Localizer<'a> {
    langs: Vec<Lang>,
    extractor: &'a Extractor,
}

impl<'a> Localizer<'a> {
    /// Create a new, empty Localizer
    pub fn new(extractor: &'a Extractor) -> Localizer<'a> {
        Localizer {
            langs: vec!(),
            extractor: extractor,
        }
    }

    /// Add a lang to the localizer
    ///
    /// # Arguments
    ///
    /// * `lang`: the code of the language (e.g. "fr", "en", ...);
    /// * `s`: a string containing localization information. It should be foramtted
    /// similarly to gettext `mo` files.
    pub fn add_lang<S: Into<String>>(&mut self, lang: S, s: &str) -> Result<()> {
        let lang = Lang::new_from_str(lang, s)?;
        self.langs.push(lang);
        Ok(())
    }

    /// Generate the `localization_macros.rs` file.
    pub fn generate_macro_file(mut self) -> String {
        macrogen::generate_macro_file(&mut self.langs, self.extractor)
    }

    /// Write the `localization_macros.rs` file to a file.
    pub fn write_macro_file<P:AsRef<Path>>(self, file: P) -> Result<()> {
        let mut f = File::create(file.as_ref())
            .map_err(|e| Error::new(format!("Could not create file {file}: {error}",
                                            file = file.as_ref().display(),
                                            error = e)))?;
        let content = self.generate_macro_file();
        f.write_all(content.as_bytes())
             .map_err(|e| Error::new(format!("Could not write to file {file}: {error}",
                                             file = file.as_ref().display(),
                                             error = e)))?;
        Ok(())
    }
}
