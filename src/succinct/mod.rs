#![allow(dead_code,non_snake_case)]

mod poppy;

use bitintr::x86::bmi2::{pdep,bzhi};
use crate::div_rem;
use std::mem::size_of;

pub trait Access {
  fn access(self, index: usize) -> bool;
}

pub trait Select1 {
  fn select1(self, count: usize) -> usize;
}

#[inline]
pub fn select1<T: Select1>(x: T, count: usize) -> usize {
  x.select1(count)
}

pub trait Select0 {
  fn select0(self, count: usize) -> usize;
}

#[inline]
pub fn select0<T: Select0>(x: T, count: usize) -> usize {
  x.select0(count)
}

pub trait Rank {
  fn rank(self, index: usize) -> usize;
}

#[inline]
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
      fn rank(self, i: usize) -> usize { 
        bzhi(self,i as $type).count_ones() as usize 
      }
    }
    impl Access for $type {
      fn access(self, i: usize) -> bool { 
        (self & (1 << i)) != 0 
      }
    }
    impl Select0 for $type {
      #[inline]
      fn select0(self, j: usize) -> usize { pdep(1<<j,!self).trailing_zeros() as usize }
    }
    impl Select1 for $type {
      #[inline]
      fn select1(self, j: usize) -> usize { pdep(1<<j, self).trailing_zeros() as usize }
    }
    /// O(n)
    impl Rank for &Vec<$type> {
      fn rank(self, i: usize) -> usize {
        let (q,r) = div_rem(i,8*size_of::<$type>());
        (0..q).fold(0,|a,b| a + self[b].count_ones() as usize) + self[q].rank(r)
      }
    }
    /// O(1)
    impl Access for &Vec<$type> {
      fn access(self, i: usize) -> bool {
        let (q,r) = div_rem(i,8*size_of::<$type>());
        self[q].access(r) 
      }
    }
  };
}

impl_all!(impl_rank_select: u8, u16, u32, u64);
