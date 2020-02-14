use std::rc::Rc;
use crate::decoder::*;

/// leafy trees
#[derive(Clone,Debug)]
pub enum Node<T> {
  Bin(Tree<T>,Tree<T>),
  Tip(T),
  Empty
}

type Tree<T> = Rc<Node<T>>;

impl <T> Decoder for Tree<T> {
  type Item = bool;
  type Cursor = Tree<T>;
  type Value = T;
  fn decoder(&self) -> Decoded<Tree<T>,T> {
    match &*self {
      &Node::Tip(t)   => Err(Some(t)),
      &Node::Empty    => Err(None),
      _               => Ok(Rc::clone(&self))
    }
  }
  fn step(cursor: Tree<T>, dir: bool) -> Decoded<Tree<T>,T> {
    match &*cursor {
      &Node::Bin(l,r) => Ok(if dir { l } else { r }),
      _               => Err(None)
    }
  }
}
