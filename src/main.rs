mod core;

use core::file::File;

fn main() {

    let _ = File::init(None, None, None);

    println!("Hello, world!");
}
