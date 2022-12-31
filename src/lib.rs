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
//! crowbook-intl = "0.1.0"
//!
//! [dependencies]
//! crowbook-intl-runtime = "0.1.0"
//! ```
//!
//! You'll then need to create the `build.rs` file, which can look like this:
//!
//! ```rust,ignore
//! extern crate crowbook_intl;
//! use crowbook_intl::{Localizer, Extractor};
//! 
//! fn main() {
//!     // Generate a `lang/default.pot` containing strings used to call `lformat!`
//!     let mut extractor = Extractor::new();
//!     extractor.add_messages_from_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap();
//!     extractor.write_pot_file(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/default.pot")).unwrap();
//!
//!     // Generate the `localize_macros.rs` file
//!     let mut localizer = Localizer::new(&extractor);
//!     // Use env::var instead of env! to avoid problems when cross-compiling
//!     let dest_path = Path::new(&env::var("OUT_DIR").unwrap())
//!        .join("localize_macros.rs");
//!     localizer.write_macro_file(dest_path).unwrap();
//! }
//! ```
//!
//! This will create a `localize_macros.rs` at build time somewhere in `OUT_DIR`, containing the `lformat!` macro.
//! To actually use this macro, you have to create a `src/localize_macros.rs` file that includes it:
//!
//! ```rust,ignore
//! include!(concat!(env!("OUT_DIR"), "/localize_macros.rs"));
//! ```
//!
//! To use it, the last step is to modify your `src/lib/lib.rs` file:
//!
//! ```rust,ignore
//! extern crate crowbook_intl_runtime;
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
//!
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
//! (...)
//! ```
//!
//! Once *this* is done, you can use the `localize_macros::set_lang` function
//! to switch the language at runtime:
//!
//! ```rust,ignore
//! use crowbook_intl_runtime::set_lang;
//! set_lang("en");
//! println!("{}", lformat!("Hello, world!")); // prints "Hello, world!"
//! set_lang("fr");
//! println!("{}", lformat!("Hello, world!")); // prints "Bonjour le monde !"
//! ```
//!
//! # Updating your translation
//!
//! When you add new strings that need to be translated (by more calls to `lformat!`),
//! or when you change the content of existing strings, you can use [Gettext's `msgmerge` and `msgcmp`](https://www.gnu.org/software/gettext/manual/html_node/msgmerge-Invocation.html)
//! commands to update your translation. While it is not guaranteed that the formats are
//! strictly identical, it should work. (That is, it is a bug if it doesn't; but at this
//! stage, this library is absolutely not guaranteed to be bug-free.)
//!
//! # Known limitations and bugs
//!
//! * Currently, `crowbook-intl` doesn't handle correctly raw string literals in `lformat!`
//!   (they won't be translated correctly).
//! * Multiple calls to the same string, but formatted differently (e.g. using a backslash
//!   before a newline to separate a string on multiple lines) will also cause problems.
//!
//! # Warning
//!
//! In case the complexity of the operation didn't discourage you, I should warn you
//! that this library is highly experimental at this time.
//!
//! # License
//!
//! This is free software, published under the [Mozilla Public License,
//! version 2.0](https://www.mozilla.org/en-US/MPL/2.0/).


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

pub use crate::error::{Result, Error};
pub use crate::localizer::Localizer;
pub use crate::extractor::Extractor;
