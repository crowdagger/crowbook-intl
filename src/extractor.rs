// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use message::Message;
use error::{Error, Result};
use common::{find_string, escape_string};

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use regex::Regex;
use walkdir::WalkDir;

/// Struct that extracts all messages from source code and can print them
/// to a `.pot` file.
///
/// This file can then be used as a starting point to begin translation.
/// It should be relatively similar to `gettext` generated files.
///
/// # Example
///
/// ```
/// use crowbook_localize::Extractor;
/// let mut extractor = Extractor::new();
/// extractor.add_messages_from_dir("src/").unwrap();
/// println!("{}", extractor.generate_pot_file());
/// ```
///
/// # Note
///
/// This struct only add messages that are considered as needing localization,
/// that is, the first argument of calls so `lformat!` macro.
pub struct Extractor {
    messages: HashMap<String, Message>,
}

impl Extractor {
    /// Create a new, empty extractor
    pub fn new() -> Extractor {
        Extractor {
            messages: HashMap::new(),
        }
    }

    /// Add all the messages contained in a source file
    pub fn add_messages_from_file<P: AsRef<Path>>(&mut self, file: P) -> Result<()> {
        lazy_static! {
            static ref REMOVE_COMMS: Regex = Regex::new(r#"//[^\n]*"#).unwrap();
            static ref FIND_MSGS: Regex = Regex::new(r#"lformat!\("#).unwrap();
        }
        
        let filename =  format!("{}", file.as_ref().display());
        let mut f = try!(File::open(file)
                         .map_err(|e| Error::parse(format!("could not open file {}: {}",
                                                           &filename,
                                                           e))));
        let mut content = String::new();
        try!(f.read_to_string(&mut content)
            .map_err(|e| Error::parse(format!("could not read file {}: {}",
                                              &filename,
                                              e))));
        content = REMOVE_COMMS.replace_all(&content, "");

        for caps in FIND_MSGS.captures_iter(&content) {
            let (_, pos) = caps.pos(0).unwrap();
            let line = 1 + &content[..pos].bytes().filter(|b| b == &b'\n').count();
            
            let bytes = content[pos..].as_bytes();
            let msg: String = escape_string(
                try!(find_string(bytes)
                      .map_err(|_| Error::parse(format!("{}:{}: could not parse as string",
                                                        &filename,
                                                        line)))))
                .into_owned();
            
            if self.messages.contains_key(msg.as_str()) {
                self.messages.get_mut(&msg).unwrap().add_source(filename.as_str(), line);
            } else {
                let mut message = Message::new(msg.as_str());
                message.add_source(filename.as_str(), line);
                self.messages.insert(msg, message);
            }
        }

        Ok(())
    }

    /// Add messages from all `.rs` files contained in a directory
    /// (walks through subdirectories)
    pub fn add_messages_from_dir<P: AsRef<Path>>(&mut self, dir: P) -> Result<()> {
        let filtered =  WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path()
                 .to_string_lossy()
                 .into_owned())
            .filter(|s| s.ends_with(".rs"));
        for filename in filtered {
            try!(self.add_messages_from_file(&filename));
        }

        Ok(())
    }

    /// Generate a pot-like file from the strings extracted from all files (if any)
    pub fn generate_pot_file(&self) -> String {
        let mut output = String::from(POT_HEADER);
        for value in self.messages.values() {
            output.push_str(&format!("{}", value));
        }
        output
    }

    /// Write a pot-like file to specified location
    pub fn write_pot_file(self, file: &str) -> Result<()> {
        let mut f = try!(File::create(file).map_err(|e| Error::new(format!("Could not create file {}: {}",
                                                                              file, e))));
        let content = self.generate_pot_file();
        try!(f.write_all(content.as_bytes())
             .map_err(|e| Error::new(format!("Could not write to file {}: {}",
                                             file, e))));
        Ok(())
    }
}

const POT_HEADER: &'static str = r#"# SOME DESCRIPTIVE TITLE
# Copyright (C) YEAR THE PACKAGE'S COPYRIGHT HOLDER
# LICENSE
# AUTHOR <EMAIL@ADDRESS>, YEAR.
#
#, fuzzy
msgid ""
msgstr ""
"Content-Type: text/plain; charset=UTF-8\n"

"#;
