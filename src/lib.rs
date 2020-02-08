#![allow(dead_code,unused_variables)]

extern crate bitintr;
use bitintr::x86::bmi::{bextr};
use bitintr::x86::bmi2::{pdep,bzhi};
use std::iter::{IntoIterator,FromIterator};

pub trait Select1 {
  fn select1(self, count: usize) -> usize;
}

pub fn select1<T: Select1>(x: T, count: usize) -> usize {
  x.select1(count)
}

pub trait Select0 {
  fn select0(self, count: usize) -> usize;
}

pub fn select0<T: Select0>(x: T, count: usize) -> usize {
  x.select0(count)
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
    impl Rank for $type {
      #[inline]
      fn rank(self, i: usize) -> usize { bzhi(self,i as $type).count_ones() as usize }
    }
    impl Select0 for $type {
      #[inline]
      fn select0(self, j: usize) -> usize {
        pdep(1 << j,!self).trailing_zeros() as usize
      }
    }
    impl Select1 for $type {
      #[inline]
      fn select1(self, j: usize) -> usize { pdep(1 << j, self).trailing_zeros() as usize }
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

#[derive(Debug,Copy,Clone)]
struct Idx(u32,u32);

impl Idx {
  /// subblock 0..=3
  #[inline]
  pub fn base(self,i: u32) -> u32 {
    let m = bzhi(self.1,10*i);
    self.0+(m&1023)+bextr(m,10,10)+bextr(m,20,10)
  }

  // unused
  pub fn unpack(self) -> (u32,u32,u32,u32) {
    (self.0,self.1&1023,bextr(self.1,10,10),bextr(self.1,20,10))
  }
}

// compact rank structure, ~.03 bits per bit storage overhead, O(1) rank
pub struct Poppy {
  raw: Vec<u64>,    // bits stored as u64s
  huge: Vec<usize>, // for every fraction of 2^31 bits
  index: Vec<Idx>   // for every fraction of 2^11 bits
}

impl Poppy {
  pub fn new(raw: Vec<u64>) -> Poppy {
    let len = raw.len();
    let step = |i:usize,k:usize,index: &mut Vec<Idx>,acc: u32| -> u32 {
      let (mut sub,i5) = ([0u32;4],i<<5);
      for j in 0..k { sub[j>>3] += raw[i5+j].count_ones() }
      index.push(Idx(acc,sub[0]+(sub[1]<<10)+(sub[2]<<20)));
      acc + sub.iter().sum::<u32>()
    };
    let steps = |n:usize,k:usize,index: &mut Vec<Idx>| (n..n+k).fold(0,|a,i| step(i,32,index,a));
    let mut index: Vec<Idx> = Vec::with_capacity((len+(1<<5)-1)>>5);
    let mut huge: Vec<usize> = Vec::with_capacity((len+(1<<25)-1)>>25);
    let hq = len>>25;
    let ha = (0..hq).fold(0,|ha,h| { huge.push(ha); ha + steps(h<<20,1<<20,&mut index) as usize });
    if bzhi(len,25) != 0 {
      huge.push(ha);
      let a = steps(hq<<20,bextr(len,5,20),&mut index);
      let r = len&31;
      if r != 0 { step(len>>5,r,&mut index,a); }
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
  fn rank(self, i: usize) -> usize {
    let (w,z) = ((i>>9)<<3, bextr(i,6,3)); // first word in the current selected subblock, word we're interested in in the subblock
    let r = (0..z).fold(0,|a,b| a+self.raw[w+b].count_ones());
    self.huge[i>>31] + (self.index[i>>11].base(bextr(i as u32,9,2)) + r) as usize + self.raw[w+z].rank(i&63)
  }
}

// Assumes lo <= hi. returns hi if the predicate is never true over [l..h)
fn search<P>(mut lo: usize, mut hi: usize, p: P) -> usize where P: Fn(usize) -> bool {
  loop {
    if lo >= hi { return lo; }
    let hml = hi-lo;
    let mid = lo + (hml>>1) + (hml>>6); // offset binary search to fix cpu k-way set associative cache alignment issues at scale
    if p(mid) { hi = mid; }
    else { lo = mid+1; }
  }
}

/*
fn select1_block(u64: meta, index : usize, block: usize) -> usize {
  let t0 = m & 1023;
  if t0 > remainder { return (0,0); }
  let t1 = t0 + bextr(m,10,10);
  if t1 > remainder { return (t0,1); }
  let t2 = t1 + bextr(m,20,10);
  if t2 > remainder { return (t1,2); }
  return (t2,3);
}

fn select1_subblock(poppy: &Poppy, index: usize, subblock: usize) -> usize {
  let word = subblock<<3
  // (word..word+8) // TODO: find the first entry where we can select within this word successfully using select, subtracting popcnt and moving to next if failed
  0usize // placeholder
}

impl Select1 for &Poppy {
  fn select1(self, i: mut usize) -> usize {
    let huge_index  = search(1,self.huge.len(),|m| self.huge[m] > i) - 1;
    let huge_base   = self.huge[huge_index];
    let block_bound = ((i-huge_base+1)<<32)-1;
    let block_index = search((huge_index<<26)+1,min((huge_index+1)<<26,self.index.len()),|m| (self.index[m] > block_bound)) - 1;
    let m = self.index[block_index];
    let block_base = huge_base + (m>>32) as usize;
    let r = i - block_base
    let (d,b) = select1_block(m, r, block_index)
    block_base + b + select1_subblock(self, r - b,  block_index << 2 + d)
  }
}
*/

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(0b00101u64.select1(1),2);
    assert_eq!(0b11101u64.rank(1),1);
  }
}
