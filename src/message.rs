// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Represents a comment concerning the location/translation of a message
#[derive(Debug)]
enum Comment {
    /// File and line
    Source(String, usize)
}


/// Represents a message, with a string and a list of comments
/// corresponding to position in source file
#[derive(Debug)]
pub struct Message {
    msg: String,
    comments: Vec<Comment> 
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
