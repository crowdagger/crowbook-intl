// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lang::Lang;
use extractor::Extractor;

/// Generate the `lformat!` macro
pub fn generate_lformat(langs: &mut [Lang], extractor: &Extractor) -> String {
    let mut arg_variant = String::new();
    let mut noarg_variant = String::new();

    for i in 0..langs.len() {
        let (curr, rest) = langs.split_at_mut(i + 1);
        let ref mut hash = curr[i].content;

        // Write keys and translations from the po files
        for (key, value) in hash {
            let b = has_arguments(key);
            let mut inner = String::new();
            if b {
                inner.push_str(&format!("            \"{}\" => format!(\"{}\", $($arg)*),\n",
                                       curr[i].lang,
                                       value));
            } else {
                inner.push_str(&format!("            \"{}\" => format!(\"{}\"),\n",
                                       curr[i].lang,
                                       value));
            }

            for other_lang in rest.iter_mut() {
                let ref mut hash = other_lang.content;
                if let Some(value) = hash.remove(key) {
                    if b {
                        inner.push_str(&format!("            \"{}\" => format!(\"{}\", $($arg)*),\n",
                                                other_lang.lang,
                                                value));
                    } else {
                        inner.push_str(&format!("            \"{}\" => format!(\"{}\"),\n",
                                                        other_lang.lang,
                                                        value));
                    }
                }
            }

            
            if b {
                inner.push_str(&format!("            _ => format!(\"{}\", $($arg)*),\n",
                                        key));
            } else {
                inner.push_str(&format!("            _ => format!(\"{}\"),\n",
                                        key));
            }
            
            let this_variant = format!("        let __guard = $crate::__get_lang();
        match __guard.as_str() {{
{}        }}",
            inner);

            if b {
                arg_variant.push_str(&format!("    (\"{}\", $($arg:tt)*) => ({{
{}
    }});\n",
                                              key, this_variant));
            } else {
                noarg_variant.push_str(&format!("    (\"{}\") => ({{
{}
    }});\n",
                                                key, this_variant));
            }
        }

        // Add translations from exact msg formats used in lformat! to the ones
        // Used in .po files (e.g. might not have the same escape codes)
        for (key, value) in extractor.original_strings() {
            if has_arguments(key) {
                arg_variant.push_str(&format!("    (\"{}\", $($arg:tt)*) => (lformat!(\"{}\", $($arg)*));\n",
                                              key, value));
            } else {
                noarg_variant.push_str(&format!("    (\"{}\") => (lformat!(\"{}\"));\n",
                                              key, value));
            }
        }
    }

    format!("/// Localized format macro (or `lformat!` in short)
///
/// Should be similar to `format!`, except strings are localized.
/// Generated automatically, you should not edit it.
#[macro_export] macro_rules! lformat {{
{}{}    ($($arg:tt)*) => (format!($($arg)*));
}}",
            &arg_variant,
            &noarg_variant)
}


/// Generate the file containing the localization macros
pub fn generate_macro_file(langs: &mut [Lang], extractor: &Extractor) -> String {
    let mut output = String::from(include_str!("../data/localize_macros.rs"));
    output.push_str(&generate_lformat(langs, extractor));
    output
}


/// Returns true if s contains arguments, false else
fn has_arguments(s: &str) -> bool {
    let chars:Vec<_> = s.chars().collect();
    for i in 0..chars.len() {
        let c = chars[i];
        if c == '{' || c == '}' {
            if i >= chars.len() - 1 {
                return true;
            } else {
                let next_c = chars[i+1];
                return !(c == next_c);
            }
        }
    }
    false
}

#[test]
fn test_arguments() {
    assert_eq!(has_arguments("foo bar"), false);
    assert_eq!(has_arguments("foo {}"), true);
    assert_eq!(has_arguments("foo {{bar}}"), false);
}
