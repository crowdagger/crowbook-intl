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
//! ```toml
//! build = "build.rs"
//! 
//! [build-dependencies]
//! crowbook-localize = "0.0.8"
//!
//! [dependencies]
//! lazy_static = "0.2" # the generated file needs `lazy_static!`
//! ```
//!
//! You'll then need to create the `build.rs` file, which can look like this:
//!
//! ```rust,ignore
//! extern crate crowbook_localize;
//! use crowbook_localize::{Localizer, Extractor};
//! 
//! fn main() {
//!     // Generate a `lang/default.pot` containing strings used to call `lformat!`
//!     let mut extractor = Extractor::new();
//!     extractor.add_messages_from_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap();
//!     extractor.write_pot_file(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/default.pot")).unwrap();
//!
//!     // Generate the `localize_macros.rs` file
//!     let mut localizer = Localizer::new(&extractor);
//!     localizer.write_macro_file(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib/localize_macros.rs")).unwrap();
//! }
//! ```
//!
//! This way, a `localize_macros.rs` file will be created at build time in `src/lib`.
//! To use it, the last step is to modify your `src/lib/lib.rs` file:
//!
//! ```rust,ignore
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
//! ```rust,ignore
//! println!("{}", lformat!("Hello, world!"));
//! ```
//! and you want it translated in french, you'll have to create a `lang/fr.po` file
//! from the `lang/default.pot` file containing:
//!
//! ```text
//! msgid "Hello, world!";
//! msgstr "Bonjour le monde !";
//! ```
//!
//! And load it in your `build.rs` file:
//!
//! ```rust,ignore
//! let mut localizer = Localizer::new();
//! localizer.add_lang("fr", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/fr.mp"))).unwrap();
//! localizer.write_macro_file("...");
//! ```
//!
//! Once *this* is done, you can use the `localize_macros::set_lang` function
//! to switch the language at runtime:
//!
//! ```rust,ignore
//! use localize_macros::set_lang;
//! set_lang("en");
//! println!("{}", lformat!("Hello, world!")); // prints "Hello, world!"
//! set_lang("fr");
//! println!("{}", lformat!("Hello, world!")); // prints "Bonjour le monde !"
//! ```
//!
//! # Updating your translation
//!
//! When you add new strings that need to be translated (by more calls to `lformat!`),
//! or when you change the content of existing strings, you can use [Gettext's `msgmerge`](https://www.gnu.org/software/gettext/manual/html_node/msgmerge-Invocation.html)
//! command to update your translation. While it is not guaranteed that the formats are
//! strictly identicals, it should work. (That is, it is a bug if it doesn't; but at this
//! stage, this library is absolutely not guaranteed to be bug-free.)
//!
//! # Warning
//!
//! In case the complexity of the operation didn't discourage you, I should warn you
//! that this library is highly experimental at this time.


extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate walkdir;

mod common;
mod macrogen;
mod lang;
mod error;
mod localizer;
mod message;
mod extractor;

pub use error::{Result, Error};
pub use localizer::Localizer;
pub use extractor::Extractor;
