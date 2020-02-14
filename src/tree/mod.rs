pub mod rc;
pub mod arena;
pub mod jacobson;

use std::marker::PhantomData;
use crate::codec::decoder::*;

// binary leafy tree base functor
#[derive(Copy,Clone,Debug)]
pub enum View<Id,T> {
  Bin(Id,Id),
  Tip(T)
}

pub trait Store<T:Clone> {
  type Id : Clone;
  fn at(&self, i: Self::Id) -> View<Self::Id,T>;
}

pub trait MutableStore<T:Clone> : Store<T> {
  fn tip(&mut self, item: T) -> Self::Id;
  fn bin(&mut self, l: Self::Id, r: Self::Id) -> Self::Id;
}

// tree, TODO: generalize to any instance of Store
#[derive(Clone,Debug)]
pub struct Tree<S:Store<T>,T:Clone> {
  store: S,
  head: S::Id,
  phantom: PhantomData<T>
}

impl <S:Store<T>,T:Clone> Decoder for Tree<S,T> where 
{
  type Symbol = bool;
  type Value = T;
  type Cursor = View<S::Id,T>;
  fn decoder(&self) -> Self::Cursor { self.store.at(self.head.clone()) }
  fn step(&self,cursor: Self::Cursor, next: bool) -> Option<Self::Cursor> {
    match cursor {
      View::Bin(l,r) => { Some(self.store.at(if next { l } else { r })) }
      _ => None
    }
  }
  fn value(&self,cursor: Self::Cursor) -> Option<T> {
    match cursor {
      View::Tip(t) => Some(t),
      _ => None
    }
  }
}