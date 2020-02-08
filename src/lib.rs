#![allow(dead_code,unused_variables)]

extern crate bitintr;
use bitintr::x86::bmi::{bextr};
use bitintr::x86::bmi2::{pdep,bzhi};
use std::iter::{IntoIterator,FromIterator};
// use std::mem::{self, MaybeUninit};

pub trait Select {
  fn select(self, count: usize) -> usize;
}

pub trait Rank {
  fn rank(self, index: usize) -> usize;
}

//macro_rules! array_from_fn {
//  ($type:ty, $size:expr, $func:expr) => {
//    unsafe {
//      let func = $func;
//      let mut array: [MaybeUninit<$type>; $size] = MaybeUninit::uninit().assume_init();
//      for i in 0..$size {
//        std::ptr::write(&mut array[i], MaybeUninit::new(func(i))); // on any panic in here we leak, don't panic
//      }
//      mem::transmute::<_, [$type; $size]>(array)
//    }
//  };
//}

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

fn make_poppy(raw: &Vec<u64>) -> Vec<u64> {
  let len = raw.len();
  let (q,r) = (len>>5,len&31);
  let mut prefix_sum = 0u64;
  let mut result: Vec<u64> = (0..q).map(|i| {
    let i5 = i<<5;
    //let sub:[u32;4] = array_from_fn!(u32,4,|j|(0usize..8usize).fold(0u32,|a,k|a+(raw[i5+(j<<3)+k].count_ones())));
    let mut sub:[u32;4] = [0;4];
    for j in 0..32 { sub[j>>3] += raw[i5+j].count_ones() }
    let result = (sub[0] + (sub[1]<<10) + (sub[2]<<20)) as u64 + (prefix_sum<<32);
    prefix_sum += (sub[0] + sub[1] + sub[2] + sub[3]) as u64;
    result
  }).collect();
  if r != 0 { // deal with any partial blocks at the end
    let mut sub:[u32;4] = [0;4];
    for j in 0..r { sub[j>>3] += raw[q*32+j].count_ones() }
    result.push((sub[0] + (sub[1]<<10) + (sub[2]<<20)) as u64 + (prefix_sum<<32));
  }
  // assert prefix_sum is 32 bits or less
  assert_eq!(prefix_sum,bzhi(prefix_sum,32));
  result
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
