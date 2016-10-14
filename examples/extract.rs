extern crate crowbook_localize;

use crowbook_localize::Extractor;

fn main() {
    let mut extractor = Extractor::new();
    extractor.add_messages_from_dir("src/").unwrap();
    println!("{}", extractor.generate_pot_file());
}
