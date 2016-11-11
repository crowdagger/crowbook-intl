// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;
use common::escape_string;

/// Represents a comment concerning the location/translation of a message
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comment {
    /// File and line
    Source(String, usize)
}


/// Represents a message, with a string and a list of comments
/// corresponding to position in source file
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message {
    pub comments: Vec<Comment>,
    pub msg: String,
}


impl Message {
    /// Creates a new message
    pub fn new<S:Into<String>>(msg: S) -> Message {
        Message {
            msg: msg.into(),
            comments: vec!(),
        }
    }

    /// Add a source location to a comment
    pub fn add_source<S:Into<String>>(&mut self, file: S, line: usize) -> &mut Self {
        self.comments.push(Comment::Source(file.into(), line));
        self
    }
}


impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#:")?;
        for comment in &self.comments {
            match *comment {
                Comment::Source(ref file, line) => write!(f, " {}:{}", file, line)?,
            }
        }
        writeln!(f, "
msgid \"{}\"
msgstr \"\"\n",
                 escape_string(self.msg.as_str()))
    }
}
