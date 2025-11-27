use super::super::dir_parser;


#[test]
fn meow() {
    let meow = dir_parser::parse_dir(String::from(""));
    println!("I survived")
}

