//! This file was generated automatically by crowbook-localize.
//! It is probably not a good idea to edit it manually.
//!
//! # Usage:
//!
//! ```rust, no_run
//! #[macro_use] mod localize_macros;
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
pub fn get_lang() -> RwLockReadGuard<'static, String> {
    LANG.read().unwrap()
}

/// Localized format macro (or `lformat!` in short)
/// Should be similar to `format!`, except strings are localized
#[macro_export] macro_rules! lformat {
    ($msg:expr) => ({
        let __guard = $crate::localize_macros::get_lang();
        let __lang: &str = __guard.as_str();
        localize!(__lang, $msg)
    });
    ($msg:expr, $($arg:tt)*) => ({
        let __guard = $crate::localize_macros::get_lang();
        let __lang: &str = __guard.as_str();
        localize!(__lang, $msg, $($arg)*)
    });
}
