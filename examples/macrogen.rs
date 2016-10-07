extern crate crowbook_localize;

use crowbook_localize::Localizer;

fn main() {
    let str_fr = r#"
msgid "hello, {}"
msgstr "bonjour, {}"

msgid "Shit: \"{}\" went wrong;"
msgstr "Chiotte: \"{}\" est parti en live;"

msgid "kwak!"
msgstr "coin !"
"#;

    let str_es = r#"
msgid "hello, {}"
msgstr "hola, {}"

msgid "Oi!"
msgstr "¡Oi!"
"#;
    let mut localizer = Localizer::new();
    localizer.add_lang("fr", str_fr).unwrap();
    localizer.add_lang("es", str_es).unwrap();
    println!("{}", localizer.generate_macro_file());
}
