extern crate distrust;
use distrust::*;

fn main() {
  println!("{:?}",0b00101u64.select(1));
  println!("{:?}",0b11101u64.rank(1));
}
