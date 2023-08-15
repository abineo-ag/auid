use base64::prelude::BASE64_STANDARD;
use base64::Engine;

use auid::Uid;

fn main() {
    let i = Uid::new();
    println!("10: {}", *i);
    println!("16: {:X}", *i);
    println!("58: {}", i);
    println!("64: {}", BASE64_STANDARD.encode(i.to_be_bytes()));
}
