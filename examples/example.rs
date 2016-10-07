#[macro_use] extern crate lazy_static;

#[macro_use] mod localize_macros;

fn main() {
    localize_macros::set_lang("fr");
    println!("{}", lformat!("hello, {}", 42));
    println!("{}", lformat!("kwak!"));
    println!("{}", lformat!("Oi!"));
    localize_macros::set_lang("es");
    println!("{}", lformat!("hello, {}", 42));
    println!("{}", lformat!("kwak!"));
    println!("{}", lformat!("Oi!"));
}
