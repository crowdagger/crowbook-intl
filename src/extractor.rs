// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use message::Message;
use error::{Error, Result};

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use regex::Regex;
use rustc_serialize::json;

/// Struct that extracts all messages from source code and can print them
/// to a `.pot` equivalent
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
            let msg = try!(find_string(bytes)
                           .map_err(|_| Error::parse(format!("{}:{}: could not parse as string",
                                                            &filename,
                                                            line))));
            
            if self.messages.contains_key(&msg) {
                self.messages.get_mut(&msg).unwrap().add_source(filename.as_str(), line);
            } else {
                let mut message = Message::new(msg.as_str());
                message.add_source(filename.as_str(), line);
                self.messages.insert(msg, message);
            }
        }

        Ok(())
    }

    pub fn print_messages(&self) {
        for value in self.messages.values() {
            println!("{:?}", value);
        }
    }
}

fn find_string(bytes: &[u8]) -> Result<String> {
    let mut vec = vec!();
    let mut began = false;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => if began {
                break
            } else {
                began = true;
            },
            b'\\' => {
                i += 1;
                if i >= bytes.len() {
                    return Err(Error::new(""));
                }
                match bytes[i] {
                    b'n' => vec.push(b'\n'),
                    b't' => vec.push(b'\t'),
                    b'r' => vec.push(b'\r'),
                    _ => vec.push(bytes[i]),
                }
            },
            _ => vec.push(bytes[i]),
        }
        i += 1;
    }
    Ok(String::from_utf8(vec).unwrap())
}