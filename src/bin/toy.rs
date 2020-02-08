extern crate distrust;
use distrust::{rank,select};

fn main() {
  println!("{:?}",select(0b00101u64,1));
  println!("{:?}",rank(0b11101u64,1));
}
