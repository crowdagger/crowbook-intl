// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lang::Lang;

/// Generate the localize! macro
pub fn generate_localize(langs: &mut [Lang]) -> String {
    let mut output = String::new();

    output.push_str("macro_rules! localize {\n");

    for i in 0..langs.len() {
        let (curr, rest) = langs.split_at_mut(i + 1);
        let ref mut hash = curr[i].content;
        for (key, value) in hash {
            // Generate variant if it contains a {}
            output.push_str(&format!("    ($lang:expr, \"{}\", $($arg:tt)*) => (\n", key));
            output.push_str("        match $lang {\n");
            output.push_str(&format!("            \"{}\" => format!(\"{}\", $($arg)*),\n",
                                    curr[i].lang,
                                     value));
            for other_lang in rest.iter_mut() {
                let ref mut hash = other_lang.content;
                if let Some(value) = hash.get(key) {
                    output.push_str(&format!("            \"{}\" => format!(\"{}\", $($arg)*),\n",
                                             other_lang.lang,
                                             value));
                }
            }
            output.push_str(&format!("            _ => format!(\"{}\", $($arg)*),\n",
                                     key));
            output.push_str("        });\n");


            // Generate variant if it doesn't contain a {}
            output.push_str(&format!("    ($lang:expr, \"{}\") => (\n", key));
            output.push_str("        match $lang {\n");
            output.push_str(&format!("            \"{}\" => format!(\"{}\"),\n",
                                     curr[i].lang,
                                     value));
            for other_lang in rest.iter_mut() {
                let ref mut hash = other_lang.content;
                // This time, remove the value, so we don't display it in next iterations
                if let Some(ref value) = hash.remove(key) {
                    output.push_str(&format!("            \"{}\" => format!(\"{}\"),\n",
                                             other_lang.lang,
                                             value));
                }
            }
            output.push_str(&format!("            _ => format!(\"{}\"),\n",
                                     key));
            output.push_str("        });\n");
        }
    }

    output.push_str("    ($lang:expr, $msg:expr) => (format!($msg));\n");
    output.push_str("    ($lang:expr, $msg:expr, $($arg:tt)*) => (\n");
    output.push_str("        format!($msg, $($arg)*)\n");
    output.push_str("    );\n");
    output.push_str("}\n");
    
    output
}
