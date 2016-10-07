// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lang::Lang;

/// Generate the localize! macro
pub fn generate_localize(langs: &mut [Lang]) -> String {
    let mut output = String::new();

    output.push_str("/// This macro was generated automatically, do not edit manually\n");
    output.push_str("#[macro_export]\n");
    output.push_str("macro_rules! localize {\n");
    output.push_str("    ($lang:expr, $msg:expr, $($arg:tt)*) => (\n");
    output.push_str("        match ($lang, $msg) {\n");

    let mut noarg_variant = String::new();
    noarg_variant.push_str("    ($lang:expr, $msg: expr) => (\n");
    noarg_variant.push_str("        match ($lang, $msg) {\n");
    
    for i in 0..langs.len() {
        let (curr, rest) = langs.split_at_mut(i + 1);
        let ref mut hash = curr[i].content;
        for (key, value) in hash {
            let b = has_arguments(key);
            if b {
                output.push_str(&format!("            (\"{}\", \"{}\") => format!(\"{}\", $($arg)*),\n",
                                         curr[i].lang,
                                         key,
                                         value));
            } else {
                noarg_variant.push_str(&format!("            (\"{}\", \"{}\") => format!(\"{}\"),\n",
                                                curr[i].lang,
                                                key,
                                                value));                
            }
            for other_lang in rest.iter_mut() {
                let ref mut hash = other_lang.content;
                if let Some(value) = hash.remove(key) {
                    if b {
                        output.push_str(&format!("            (\"{}\", \"{}\") => format!(\"{}\", $($arg)*),\n",
                                                 other_lang.lang,
                                                 key,
                                                 value));
                    } else {
                        noarg_variant.push_str(&format!("            (\"{}\", \"{}\") => format!(\"{}\"),\n",
                                                        other_lang.lang,
                                                        key,
                                                        value));
                    }
                }
            }
            if b {
                output.push_str(&format!("            (_, \"{}\") => format!(\"{}\", $($arg)*),\n",
                                         key,
                                         key));
            } else {
                noarg_variant.push_str(&format!("            (_, \"{}\") => format!(\"{}\"),\n",
                                                key,
                                                key));
            }
        }
    }
    output.push_str("            (_, _) => format!($msg, $($arg)*),\n");
    output.push_str("        });\n");
    noarg_variant.push_str("            (_, _) => format!($msg),\n");
    noarg_variant.push_str("        });\n");
    output.push_str(&noarg_variant);
    output.push_str("}\n");
    
    output
}


/// Generate the file containing the localization macros
pub fn generate_macro_file(langs: &mut [Lang]) -> String {
    let mut output = String::from(include_str!("../data/localize_macros.rs"));
    output.push_str(&generate_localize(langs));
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
