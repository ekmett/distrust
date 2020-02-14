use std::rc::Rc;
use crate::codec::decoder::*;
//use crate::util::unless;
use crate::tree::*;

#[derive(Clone,Debug)]
pub struct Arena<T> {
  bins: Vec<(PackedId,PackedId)>,
  tips: Vec<T>
}

impl <T:Clone> Store<T> for Arena<T> {
  type Id = PackedId;
  fn at(&self,p: PackedId) -> View<PackedId,T> {
    match p.unpack() {
      UnpackedId::Bin(i) => { let (l,r) = self.bins[i]; View::Bin(l,r) },
      UnpackedId::Tip(i) => View::Tip(self.tips[i].clone())
    }
  }
}

impl <T:Clone> MutableStore<T> for Arena<T> {
  fn tip(&mut self,item: T) -> PackedId {
    let id = self.tips.len();
    self.tips.push(item);
    PackedId::tip(id)
  }
  fn bin(&mut self,left : PackedId, right: PackedId) -> PackedId {
    let id = self.bins.len();
    self.bins.push((left,right));
    PackedId::bin(id)
  }
}

#[derive(Copy,Clone,Debug)]
pub enum UnpackedId {
  Bin(usize),
  Tip(usize)
}

impl UnpackedId {
  pub fn pack(self) -> PackedId {
    match self {
      UnpackedId::Bin(i) => PackedId::bin(i),
      UnpackedId::Tip(i) => PackedId::tip(i)
    }
  }
}

#[derive(Copy,Clone,Debug)]
pub struct PackedId(u32);
impl PackedId {
  fn unpack(self) -> UnpackedId {
    let i = (self.0 >> 1) as usize;
    if (self.0 & 1) == 1 {
      UnpackedId::Bin(i)
    } else {
      UnpackedId::Tip(i)
    }
  }
  fn tip(i: usize) -> PackedId {
    PackedId((i<<1) as u32)
  }
  fn bin(i: usize) -> PackedId {
    PackedId(((i<<1)+1) as u32)
  }
}

#[derive(Clone,Debug)]
pub struct ArenaTree<T> {
  arena: Rc<Arena<T>>,
  head: View<PackedId,T>
}

impl <T:Copy> Decoder for ArenaTree<T> {
  type Symbol = bool;
  type Value = T;
  type Cursor = ArenaTree<T>;
  fn decoder(&self) -> ArenaTree<T> { self.clone() } // panic!("heh") }  // self.clone() }
  fn step(cursor: &mut ArenaTree<T>, next: bool) -> bool {
    match cursor.head {
      View::Bin(l,r) => { cursor.head = cursor.arena.as_ref().at(if next { l } else { r }); true }
      _ => false
    }
  }
  fn value(cursor: ArenaTree<T>) -> Option<T> {    
    match cursor.head {
      View::Tip(t) => Some(t),
      _ => None
    }
  }
}