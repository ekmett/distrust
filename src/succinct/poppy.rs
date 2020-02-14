#![allow(dead_code,non_snake_case)]

// extern crate bitintr;

use bitintr::x86::bmi::bextr;
use bitintr::x86::bmi2::bzhi;
use crate::binary_search;
use crate::succinct::*;
use std::cmp::min;
use std::iter::{IntoIterator,FromIterator};

#[derive(Debug,Copy,Clone)]
struct Idx(u32,u32);

impl Idx {
  /// subblock 0..=3
  #[inline]
  pub fn base(self,i: u32) -> u32 {
    let m = bzhi(self.1,10*i);
    self.0+(m&1023)+bextr(m,10,10)+bextr(m,20,10)
  }
}

// compact rank structure, ~.03 bits per bit storage overhead, O(1) rank
pub struct Poppy(Vec<usize>,Vec<Idx>,Vec<u64>);

impl Poppy {
  pub fn new(raw: Vec<u64>) -> Poppy {
    let N = raw.len();
    let step = |n:usize,k:usize,idx:&mut Vec<Idx>,acc:u32| {
      let mut sub = [0;4];
      for i in 0..k { sub[i>>3] += raw[n+i].count_ones() }
      idx.push(Idx(acc,sub[0]+(sub[1]<<10)+(sub[2]<<20)));
      acc+sub.iter().sum::<u32>()
    };
    let steps = |n:usize,k:usize,idx:&mut Vec<Idx>| (n..n+k).fold(0,|a,i|step(i<<5,32,idx,a));
    let cap = |k:usize|(N+(1<<k)-1)>>k;
    let mut idx = Vec::with_capacity(cap(5));
    let mut big = Vec::with_capacity(cap(25));
    let hq = N>>25;
    let ha = (0..hq).fold(0,|ha,h|{ big.push(ha); ha + steps(h<<20,1<<20,&mut idx) as usize });
    if bzhi(N,25) != 0 {
      big.push(ha);
      let a = steps(hq<<20,bextr(N,5,20),&mut idx);
      let r = N&31;
      if r != 0 {
        step(N&!31,r,&mut idx,a);
      }
    }
    Poppy(big,idx,raw)
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
    self.2.into_iter()
  }
}

impl Rank for &Poppy {
  fn rank(self, i: usize) -> usize {
    let (w,z) = ((i>>9)<<3, bextr(i,6,3)); // first word in the current selected subblock, word we're interested in in the subblock
    let r = (0..z).fold(0,|a,b| a+self.2[w+b].count_ones());
    self.0[i>>31] + (self.1[i>>11].base(bextr(i as u32,9,2)) + r) as usize + self.2[w+z].rank(i&63)
  }
}

impl Access for &Poppy {
  fn access(self, i: usize) -> bool { self.2.access(i) }
}

fn select1_block(m:usize,i:usize) -> (usize,usize) {
  let t0 = m & 1023;
  if t0 > i { return (0,0); }
  let t1 = t0 + bextr(m,10,10);
  if t1 > i { return (t0,1); }
  let t2 = t1 + bextr(m,20,10);
  if t2 > i { return (t1,2); }
  return (t2,3);
}

impl Select1 for &Poppy {
  /// O(log n)
  fn select1(self, mut i: usize) -> usize {
    let hi = binary_search(1,self.0.len(),|m| self.0[m] > i) - 1;
    let mut o = self.0[hi];
    i -= o;
    let bi = binary_search((hi<<25)+1,min(self.1.len(),(hi+1)<<25),|m| (self.1[m].0 as usize > i)) - 1;
    let m = self.1[bi];
    let bd = m.0 as usize;
    o += bd; i -= bd;
    let (sd,si) = select1_block(m.1 as usize, i); // now we've found the subblock
    o += sd; i -= sd;
    let mut p = (bi<<5)|(si<<3);
    loop { // linear final scan
      let w = self.2[p];
      let d = w.count_ones() as usize;
      if d > i {
        return o + select1(w,i);
      }
      o += d; i -= d; p += 1;
    }
  }
}
