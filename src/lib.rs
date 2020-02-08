#![allow(dead_code)]
#![allow(unused_variables)]
extern crate bitintr;
use bitintr::x86::bmi2::{pdep,bzhi};

pub trait Select {
  fn select(self, count: usize) -> usize;
}

pub trait Rank {
  fn rank(self, index: usize) -> usize;
}

impl Select for u64 {
  fn select(self, j: usize) -> usize {
    pdep(1u64 << j, self).trailing_zeros() as usize
  }
}

impl Rank for u64 {
  fn rank(self, i: usize) -> usize {
    bzhi(self,i as u64).count_ones() as usize
  }
}

// O(n) brute force
impl Rank for &Vec<u64> {
  fn rank(self, i: usize) -> usize {
    let q = i/64;
    (0..q).fold(0,|a,b| a + self[b].count_ones() as usize) + self[q].rank(i%64)
  }
}

struct Poppy { bits: Vec<u64>, index: Vec<u64> }

impl Poppy {
  pub fn new(bits: Vec<u64>) -> Poppy {
    let index = make_poppy(&bits);
    Poppy { bits: bits, index: index }
  }
}

fn make_poppy(bits : &Vec<u64>) -> Vec<u64> {
  vec![] // placeholder
}

impl Rank for &Poppy {
  fn rank(self, index: usize) -> usize {
    self.bits.rank(index) // placeholder
  }
}
