extern crate crowbook_localize;

use crowbook_localize::{Lang, generate_macro_file};

fn main() {
    let str_fr = r#"
msgid "hello, {}"
msgstr "bonjour, {}"

msgid "kwak!"
msgstr "coin !"
"#;

    let str_es = r#"
msgid "hello, {}"
msgstr "hola, {}"

msgid "Oi!"
msgstr "¡Oi!"
"#;
    let fr = Lang::new_from_str("fr", str_fr).unwrap();
    let es = Lang::new_from_str("es", str_es).unwrap();
    println!("{}", generate_macro_file(&mut [fr, es]));
}
