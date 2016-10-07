//! This file was generated automatically by crowbook-localize.
//! It is probably not a good idea to edit it manually.
//!
//! # Usage:
//!
//! ```rust, no_run
//! #[macro_use] mod localize_macros;
//! use localize_macros::set_lang;
//! set_lang("en");
//! lformat!("Hello, {}", name);
//! set_lang("fr");
//! lformat!("Hello, {}", name);
//! ```

use std::sync::RwLock;
use std::sync::RwLockReadGuard;

lazy_static! {
    pub static ref LANG: RwLock<String> = RwLock::new(String::from("en"));
}

/// Sets the lang
pub fn set_lang<S>(lang: S)
    where S: Into<String> {
    *LANG.write().unwrap() = lang.into();
}

/// Get the lang (or a guard on it)
///
/// This function should not be used directly
#[doc(hidden)]
pub fn __get_lang() -> RwLockReadGuard<'static, String> {
    LANG.read().unwrap()
}

