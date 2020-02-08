#![allow(dead_code,unused_variables)]

extern crate bitintr;
use bitintr::x86::bmi::{bextr};
use bitintr::x86::bmi2::{pdep,bzhi};
use std::iter::{IntoIterator,FromIterator};

pub trait Select {
  fn select(self, count: usize) -> usize;
}

pub fn select<T: Select>(x: T, count: usize) -> usize {
  x.select(count)
}

pub trait Rank {
  fn rank(self, index: usize) -> usize;
}

pub fn rank<T: Rank>(x: T, index: usize) -> usize {
  x.rank(index)
}

macro_rules! impl_all {
  ($impl_macro:ident: $($id:ident),*) => { $($impl_macro!($id);)* }
}

macro_rules! impl_rank_select {
  ($type:ty) => {
    impl Select for $type {
      #[inline]
      fn select(self, j: usize) -> usize { pdep(1 << j, self).trailing_zeros() as usize }
    }
    impl Rank for $type {
      #[inline]
      fn rank(self, i: usize) -> usize { bzhi(self,i as $type).count_ones() as usize }
    }
    /// O(n)
    impl Rank for &Vec<$type> {
      fn rank(self, i: usize) -> usize {
        const BITS: usize = std::mem::size_of::<$type>()*8;
        let q = i/BITS;
        (0..q).fold(0,|a,b| a + self[b].count_ones() as usize) + self[q].rank(i%BITS)
      }
    }
  };
}

impl_all!(impl_rank_select: u8, u16, u32, u64);

// compact rank structure, ~.03 bits per bit storage overhead
pub struct Poppy { raw: Vec<u64>, huge: Vec<u64>, index: Vec<u64> }

impl Poppy {
  pub fn new(raw: Vec<u64>) -> Poppy {
    let len = raw.len();
    let (q,r,hq) = (len>>5,len&31,len>>26);
    let step = |block:usize,k:usize,index: &mut Vec<u64>,sum: u64| -> u64 {
      let mut sub = [0u32;4];
      for j in 0..k { sub[j>>3] += raw[(block<<5)+j].count_ones() }
      index.push((sub[0]+(sub[1]<<10)+(sub[2]<<20)) as u64 + (sum<<32));
      sum+((sub[0]+sub[1]+sub[2]+sub[3]) as u64)
    };
    let mut index: Vec<u64> = Vec::with_capacity((len+(1<<5)-1)>>5);
    let mut huge: Vec<u64> = Vec::with_capacity((len+(1<<26)-1)>>26);
    let huge_sum = (0..hq).fold(0,|huge_acc,h| {
      huge.push(huge_acc);
      let h21 = h<<21;
      huge_acc + (0..1<<21).fold(0,|block_acc,i| step(h21+i,32,&mut index, block_acc))
    });
    if bzhi(len,26) != 0 {
      huge.push(huge_sum);
      let hq21 = hq<<21;
      let block_sum = (0..bextr(len,5,21)).fold(0,|block_acc,i| step(hq21+i,32,&mut index, block_acc));
      if r != 0 {
        step(q,r,&mut index,block_sum);
      }
    }
    Poppy{raw,huge,index}
  }
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

impl Rank for &Poppy {
  fn rank(self, i : usize) -> usize {
    let w = self.index[i >> 11]; // raw word from the index
    let base = bextr(i,9,23) << 3; // first word in the current selected subblock
    let z = bextr(i,6,3); // how many u64s do I need to popcount in the subblock
    let word_rank = (0..z).fold(0,|a,b| a+self.raw[base + b].count_ones()) as usize;
    let m = bzhi(w as u32, 10*bextr(i,9,2) as u32);
    let subblock_rank = ((m&1023) + bextr(m,10,10) + bextr(m,20,10)) as usize;
    let block_rank = (self.huge[i >> 32] + (w >> 32)) as usize; // allow structures > 4gb
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
