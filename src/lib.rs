#![allow(dead_code,unused_variables)]

extern crate bitintr;
use bitintr::x86::bmi::{bextr};
use bitintr::x86::bmi2::{pdep,bzhi};
use std::iter::{IntoIterator,FromIterator};

pub trait Select {
  fn select(self, count: usize) -> usize;
}

pub trait Rank {
  fn rank(self, index: usize) -> usize;
}

macro_rules! impl_all {
  ($impl_macro:ident: $($id:ident),*) => {
    $(
      $impl_macro!($id);
    )*
  }
}

macro_rules! impl_rank_select {
  ($id:ident) => {
    impl Select for $id {
      #[inline]
      fn select(self, j: usize) -> usize { pdep(1 << j, self).trailing_zeros() as usize }
    }
    impl Rank for $id {
      #[inline]
      fn rank(self, i: usize) -> usize { bzhi(self,i as $id).count_ones() as usize }
    }
    impl Rank for &Vec<$id> {
      fn rank(self, i: usize) -> usize {
        let q = i/64;
        (0..q).fold(0,|a,b| a + self[b].count_ones() as usize) + self[q].rank(i%64)
      }
    }
  };
}

impl_all!(impl_rank_select: u8, u16, u32, u64);

// small poppy structure, represents up to 2^32 entries
struct Poppy { raw: Vec<u64>, index: Vec<u64> }

impl Poppy {
  pub fn new(bits: Vec<u64>) -> Poppy {
    let index = make_poppy(&bits);
    Poppy { raw: bits, index: index }
  }
//  pub fn push(&mut self, elem: u64) {
//    self.raw.push(elem)
//    let n = self.raw.len()
//    if (n & 31 == 1) {
//      // TODO: we need to push a new entry, grab the last entry, add the total for the final subblock,
//      // and init this entry in the index
//    } else if (n & 7 == 0 && n & 31 /= 0) {
//      // TODO: update our subblock count here
//    }
//  }
}

impl FromIterator<u64> for Poppy {
  fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Poppy {
    Poppy::new(Vec::from_iter(iter))
  }
}

impl IntoIterator for Poppy {
  type Item = u64;
  type IntoIter = std::vec::IntoIter<u64>;
  fn into_iter(self) -> Self::IntoIter {
    self.raw.into_iter()
  }
}

fn vec_init<T>(count: usize, f: fn(usize) -> T) -> Vec<T> {
  (0..count).map(f).collect()
}

fn make_poppy(raw : &Vec<u64>) -> Vec<u64> {
  vec![]
//  let len = raw.len();
//  let (q,r) = (len>>5,len&31);
//  let mut prefix_sum = 0usize;
//  let mut v = vec_init(raw.len()>>5,|i| {
//    i*32
//    return subblock1_sum + (subblock2_sum<<10) + (subblock3_sum<<20) + (prefix_sum << 32)
//  });
//  if r != 0 {
//    v.push(...)
//  }
}

impl Rank for &Poppy {
  fn rank(self, i : usize) -> usize {
    let w = self.index[i >> 11]; // raw word from the index
    let base = bextr(i,9,23) << 3; // first word in the current selected subblock
    let z = bextr(i,6,3); // how many u64s do I need to popcount in the subblock
    let word_rank = (0..z).fold(0u32,|a,b| a + self.raw[base + b].count_ones()) as usize;
    let m = bzhi(w as u32, 10*bextr(i,9,2) as u32);
    let subblock_rank = ((m&1023) + bextr(m,10,10) + bextr(m,20,10)) as usize;
    let block_rank = (w >> 32) as usize;
    block_rank + subblock_rank + word_rank + self.raw[base + z].rank(i & 63)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(0b00101u64.select(1),2);
    assert_eq!(0b11101u64.rank(1),1);
  }
}
