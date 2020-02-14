use std::ops::Try;

#[derive(Copy,Clone,Debug)]
pub enum Decoded<Cursor,Value> {
  Cursor(Cursor),
  Done(Value),
  Illegal
}

impl <Cursor,Value> Try for Decoded<Cursor,Value> {
  type Ok = Cursor
  type Error = Option<Value>
  fn into_result(self) -> Result<Cursor,Option<Value>> {
    match self {
      Cursor(c) -> Ok(c)
      Done(v) -> Err(Some(v))
      Illegal -> Err(None)
    }
  }
  fn from_error(mv: Option<Value>) -> Decoded<Cursor,Value> {
    match mv {
      None    -> Illegal
      Some(v) -> Done(v)
    }
  }

  fn from_ok(c: Cursor) -> Decoded<Cursor,Value> {
    Cursor(c)
  }
}

pub trait Decoder {
  type Item;
  type Cursor;
  type Value;
  fn decoder(self) -> Decoded<Cursor,Value>
  fn step(cursor: Cursor, next: Item) -> DecodeState<Cursor,Value>
  fn decode<I:Iterator<Item=Self::Item>>(self,t: I) -> Option<Value> {
    t.try_fold(self.decoder()?,|a,b|step(a,b))
  }
} 

/// leafy trees
#[derive(Copy,Clone,Debug)]
pub enum LeafyNode<T> {
  Bin(Leafy<T>,Leafy<T>),
  Tip(T),
  Empty
}

type Leafy<T> = Rc<LeafyNode<T>>

impl <T> Decoder for Leafy<T> {
  type Item = bool;
  type Cursor = Leafy<T>;
  type Value = T;
  fn decoder(self) -> Decoded<Tree<T>,T> {
    match *self {
      Tip(t)   => Decoded::Done(t)
      Empty    => Decoded::Illegal
      Bin(_,_) => Decoded::Cursor(Rc::clone(&self))
    }
  }
  fn step(cursor: Leafy<T>, dir: bool) -> DecodedState<Leafy<T>,T> {
    match *cursor {
      Bin(l,r) => Decoded::Cursor(if dir { Rc::clone(&l) } else { Rc::clone(&r))
      _        => Decoded::Illegal
    }
  }
}

pub trait Encoder {
  type Item;
  type EncodedIter: Iterator<Item = Self::Item>
  fn encode(self, val: Value) -> Self::EncodedIter
}
