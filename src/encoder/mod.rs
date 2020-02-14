use std::iter::Iterator;

pub trait Encoder {
  type Value;
  type EncodedIter: Iterator;
  fn encode(self, val: Self::Value) -> Self::EncodedIter;
}
