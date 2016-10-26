extern crate crowbook_intl;

use crowbook_intl::Extractor;

fn main() {
    let mut extractor = Extractor::new();
    extractor.add_messages_from_dir("src/").unwrap();
    println!("{}", extractor.generate_pot_file());
}
