use std::iter::Iterator;

pub trait Decoder {
  type Symbol;
  type Value;
  type Cursor;
  fn decoder(&self) -> Self::Cursor;
  fn step(&self, cursor: Self::Cursor, next: Self::Symbol) -> Option<Self::Cursor>;
  fn value(&self, cursor: Self::Cursor) -> Option<Self::Value>;
  fn decode<I:Iterator<Item=Self::Symbol>>(&self,t: &mut I) -> Option<Self::Value> {
    self.value(t.try_fold(self.decoder(),|a,i|self.step(a,i))?)
  }
}