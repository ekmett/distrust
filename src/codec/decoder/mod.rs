use std::iter::Iterator;
use crate::when;

// all codes must take 1 bit or more
pub trait Decoder {
  type Symbol;
  type Value;
  type Cursor;
  fn decoder(&self) -> Self::Cursor;
  fn step(&self, cursor: &mut Self::Cursor, next: Self::Symbol) -> bool;
  fn value(&self, cursor: Self::Cursor) -> Option<Self::Value>;
  fn decode<I:Iterator<Item=Self::Symbol>>(&self,t: &mut I) -> Option<Self::Value> {
    let mut d = self.decoder();
    when(t.all(|i|self.step(&mut d,i)))?;
    self.value(d)
  }
} 
