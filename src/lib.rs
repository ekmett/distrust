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
struct Poppy { raw: Vec<u64>, huge: Vec<u64>, index: Vec<u64> }

impl Poppy {
  pub fn new(raw: Vec<u64>) -> Poppy {
    let len = raw.len();
    let (q,r) = (len>>5,len&31);
    let mut prefix_sum = 0u64;
    let mut huge: Vec<u64> = vec![];
    let mut index: Vec<u64> = (0..q).map(|i| {
      if bzhi(i,27) == 0 { // this should be done by inverting this loop
        huge.push(prefix_sum);
        prefix_sum = 0;
      }
      let mut sub:[u32;4] = [0;4];
      for j in 0..32 { sub[j>>3] += raw[(i<<5)+j].count_ones() }
      let result = (sub[0] + (sub[1]<<10) + (sub[2]<<20)) as u64 + (prefix_sum<<32);
      prefix_sum += (sub[0] + sub[1] + sub[2] + sub[3]) as u64;
      // every 4gb we should clear the current prefix_sum and write it into the huge index
      result
    }).collect();
    if r != 0 { // deal with any partial blocks at the end
      if bzhi(q,27) == 0 { // this should be done by inverting this loop
        huge.push(prefix_sum);
        prefix_sum = 0;
      }
      let mut sub:[u32;4] = [0;4];
      for j in 0..r { sub[j>>3] += raw[(q<<5)+j].count_ones() }
      index.push((sub[0] + (sub[1]<<10) + (sub[2]<<20)) as u64 + (prefix_sum<<32));
    }
    Poppy { raw: raw, huge: huge, index: index }
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
    let block_rank = (w >> 32) as usize;
    let huge_rank = self.huge[i >> 32] as usize; // allow structures > 4gb
    huge_rank + block_rank + subblock_rank + word_rank + self.raw[base + z].rank(i & 63)
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
