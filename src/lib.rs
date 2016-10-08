// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A library to localize strings, translating them according to runtime options.
//!
//! Basically, this library allows your project to generate a `lformat!` macro, that behaves
//! similarly to `format!`, except the message string (the first argument) might get translated
//! (if you can find the appropriate string for the language).
//!
//! # Usage
//!
//! First, you'll need to add the following to your `Cargo.toml` file:
//!
//! ```toml, no_run
//! build = "build.rs"
//! 
//! [build-dependencies]
//! crowbook-localize = "0.0.2"
//!
//! [dependencies]
//! lazy_static = "0.2" # the generated file needs `lazy_static!`
//! ```
//!
//! You'll then need to create the `build.rs` file, which can look like this:
//!
//! ```rust, no_run
//! extern crate crowbook_localize;
//! use crowbook_localize::Localizer;
//! 
//! fn main() {
//!     let mut localizer = Localizer::new();
//!     
//!     localizer.write_macro_file(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib/localize_macros.rs")).unwrap();
//! }
//! ```
//!
//! This way, a `localize_macros.rs` file will be created at build time in `src/lib`.
//! To use it, the last step is to modify your `src/lib/lib.rs` file:
//!
//! ```rust, no_run
//! #[macro_use] extern crate lazy_static;
//! #[macro_use] mod localize_macros;
//! ```
//!
//! Once this is done, you can start replacing your calls to `format!` with calls to `lformat!`.
//!
//! In order to get translation, you'll need to actually translate the strings in separate
//! files, and set your `build.rs` to load them.
//!
//! E.g., if you have the following code:
//!
//! ```rust, no_run
//! println!("{}", lformat!("Hello, world!"));
//! ```
//! and you want it translated in french, you'll have to create a `lang/fr.mo` file containing:
//!
//! ```text, no_run
//! msgid "Hello, world!";
//! msgstr "Bonjour le monde !";
//! ```
//!
//! And load it in your `build.rs` file:
//!
//! ```rust, no_run
//! let mut localizer = Localizer::new();
//! localizer.add_lang("fr", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/fr.mo"))).unwrap();
//! localizer.write_macro_file("...");
//! ```
//!
//! Once *this* is done, you can use the `localize_macros::set_lang` function
//! to switch the language at runtime:
//!
//! ```rust, no_run
//! use localize_macros::set_lang;
//! set_lang("en");
//! println!("{}", lformat!("Hello, world!")); // prints "Hello, world!"
//! set_lang("fr");
//! println!("{}", lformat!("Hello, world!")); // prints "Bonjour le monde !"
//! ```
//!
//! # Warning
//!
//! In case the complexity of the operation didn't discourage you, I should warn you
//! that this library is highly experimental at this time.


extern crate rustc_serialize;
extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate walkdir;

mod macrogen;
mod lang;
mod error;
mod localizer;
mod message;
mod extractor;

pub use error::{Result, Error};
pub use localizer::Localizer;
pub use extractor::Extractor;
