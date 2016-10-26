// This file was generated automatically by crowbook-localize.
// It is probably not a good idea to edit it manually.
//
// # Usage:
//
// ```rust, no_run
// extern crate crowbook_intl_runtime;
// #[macro_use] mod localize_macros;
// crowbook_intl_runtime::set_lang("en");
// lformat!("Hello, {}", name);
// set_lang("fr");
// lformat!("Hello, {}", name);
// ```

use crowbook_intl_runtime;

use std::sync::RwLockReadGuard;

#[doc(hidden)]
pub fn __get_lang() -> RwLockReadGuard<'static, String> {
    crowbook_intl_runtime::__get_lang()
}


