use lib::nat::{_0, _5};
use lib::nat::ops::{NatAdd, ToNumber};

fn main() {
    println!("{}", _5.add(_0).to_number());
}