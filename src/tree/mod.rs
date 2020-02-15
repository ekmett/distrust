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

impl <A,T> View<A,T> {
  // linear left map
  #[inline]
  fn llmap<F,B>(self,f: F) -> View<B,T> where F : Fn(A) -> B {
    match self {
      View::Bin(l,r) => View::Bin(f(l),f(r)),
      View::Tip(t) => View::Tip(t)
    }
  }
}

pub trait Store<T:Clone> {
  type Id : Clone;
  fn at(&self, i: Self::Id) -> View<Self::Id,T>;
  fn fold<F,S>(&self, phi: &F, root: Self::Id) -> S where F : Fn(View<S,T>) -> S {
    phi(self.at(root).llmap(|x|self.fold(phi,x)))
  }
}

pub trait MutableStore<T:Clone> : Store<T> {
  fn tip(&mut self, item: T) -> Self::Id;
  fn bin(&mut self, l: Self::Id, r: Self::Id) -> Self::Id;
  // self.at(self.inj(x)) = x // but we also insert into the store
  fn inj(&mut self, s: View<Self::Id,T>) -> Self::Id {
    match s {
      View::Bin(l,r) => self.bin(l,r),
      View::Tip(t) => self.tip(t)
    }
  }
  fn unfold<F,S>(&mut self, psi: &F, s: S) -> Self::Id where F: Fn(S) -> View<S,T> {
    match psi(s) {
      View::Bin(l,r) => {
        let lp = self.unfold(psi,l);
        let rp = self.unfold(psi,r);
        self.bin(lp,rp)
      },
      View::Tip(t) => self.tip(t)
    }
  }
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