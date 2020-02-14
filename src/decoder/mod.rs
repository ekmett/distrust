use std::iter::Iterator;
use crate::util::when;

pub trait Decoding {
  type Item;
  type Value;
  fn step(&mut self, next: Self::Item) -> bool;
  fn value(&self) -> Option<Self::Value>;
}


// all codes must take 1 bit or more
pub trait Decoder {
  type Item;
  type Value;
  type Cursor:Decoding<Item=Self::Item,Value=Self::Value>;
  fn decoder(&self) -> Self::Cursor;
  fn decode<I:Iterator<Item=Self::Item>>(&self,t: &mut I) -> Option<Self::Value> {
    let mut d = self.decoder();
    when(t.all(|i|d.step(i)))?;
    d.value()
  }
} 
