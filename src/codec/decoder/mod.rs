use std::iter::Iterator;
use crate::util::when;

// all codes must take 1 bit or more
pub trait Decoder {
  type Symbol;
  type Value;
  type Cursor;
  fn decoder(&self) -> Self::Cursor;
  fn step(cursor: &mut Self::Cursor, next: Self::Symbol) -> bool;
  fn value(cursor: Self::Cursor) -> Option<Self::Value>;
  fn decode<I:Iterator<Item=Self::Symbol>>(&self,t: &mut I) -> Option<Self::Value> {
    let mut d = self.decoder();
    when(t.all(|i|Self::step(&mut d,i)))?;
    Self::value(d)
  }
} 
