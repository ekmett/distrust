#![allow(dead_code,non_snake_case)]

mod poppy;

use bitintr::x86::bmi2::{pdep,bzhi};
use crate::div_rem;
use std::mem::size_of;

pub trait Access {
  type Item;
  fn access(&self, index: usize) -> Self::Item;
}

pub trait AccessPrim : Access {
  fn access_prim(self, index: usize) -> Self::Item;
}

#[inline]
pub fn access<T: Access>(x: &T, count: usize) -> T::Item {
  x.access(count)
}

#[inline]
pub fn access_prim<T: AccessPrim>(x: T, count: usize) -> T::Item {
  x.access_prim(count)
}

pub trait Select1 {
  fn select1(&self, count: usize) -> usize;
}

pub trait Select1Prim : Select1 {
  fn select1_prim(self, count: usize) -> usize;
}

#[inline]
pub fn select1<T: Select1>(x: &T, count: usize) -> usize {
  x.select1(count)
}

#[inline]
pub fn select1_prim<T: Select1Prim>(x: T, count: usize) -> usize {
  x.select1_prim(count)
}

pub trait Select0 {
  fn select0(&self, count: usize) -> usize;
}

pub trait Select0Prim : Select0 {
  fn select0_prim(self, count: usize) -> usize;
}

#[inline]
pub fn select0<T: Select0>(x: &T, count: usize) -> usize {
  x.select0(count)
}

#[inline]
pub fn select0_prim<T: Select0Prim>(x: T, count: usize) -> usize {
  x.select0_prim(count)
}

pub trait Rank {
  type Item;
  fn rank(&self, item: Self::Item, index: usize) -> usize;
}

pub trait RankPrim : Rank {
  fn rank_prim(self, item: Self::Item, index: usize) -> usize;
}

#[inline]
pub fn rank<T: Rank>(x: &T, item: T::Item, index: usize) -> usize {
  x.rank(item,index)
}

#[inline]
pub fn rank_prim<T: RankPrim>(x: T, item: T::Item, index: usize) -> usize {
  x.rank_prim(item,index)
}

pub trait BoolRank : Rank<Item=bool> {
  fn rank0(&self, index: usize) -> usize {
    self.rank(false,index)
  }
  fn rank1(&self, index: usize) -> usize {
    self.rank(true,index)
  }
}

pub trait BoolRankPrim : BoolRank + RankPrim<Item=bool> {
  fn rank0_prim(self, index: usize) -> usize;
  fn rank1_prim(self, index: usize) -> usize;
}

#[inline]
pub fn rank0<T: BoolRank>(x: &T, index: usize) -> usize {
  x.rank0(index)
}

#[inline]
pub fn rank0_prim<T: BoolRankPrim>(x: T, index: usize) -> usize {
  x.rank0_prim(index)
}


#[inline]
pub fn rank1<T: BoolRank>(x: &T, index: usize) -> usize {
  x.rank1(index)
}

#[inline]
pub fn rank1_prim<T: BoolRankPrim>(x: T, index: usize) -> usize {
  x.rank1_prim(index)
}

macro_rules! impl_all {
  ($impl_macro:ident: $($id:ident),*) => { $($impl_macro!($id);)* }
}

macro_rules! impl_rank_select {
  ($type:ty) => {
    impl Rank for $type {
      type Item = bool;
      #[inline]
      fn rank(&self,item: bool,i: usize) -> usize {
        bzhi(if item { *self } else { !*self },i as $type).count_ones() as usize 
      }
    }
    impl RankPrim for $type {
      #[inline]
      fn rank_prim(self,item: bool,i: usize) -> usize {
        bzhi(if item { self } else { !self },i as $type).count_ones() as usize 
      }
    }
    impl BoolRank for $type {
      #[inline]
      fn rank0(&self, i: usize) -> usize { 
        bzhi(!*self,i as $type).count_ones() as usize 
      }
      #[inline]
      fn rank1(&self, i: usize) -> usize { 
        bzhi(*self,i as $type).count_ones() as usize 
      }
    }
    impl BoolRankPrim for $type {
      #[inline]
      fn rank0_prim(self, i: usize) -> usize { 
        bzhi(!self,i as $type).count_ones() as usize 
      }
      #[inline]
      fn rank1_prim(self, i: usize) -> usize { 
        bzhi(self,i as $type).count_ones() as usize 
      }
    }
    impl Access for $type {
      type Item = bool;
      #[inline]
      fn access(&self, i: usize) -> bool { 
        (self & (1 << i)) != 0 
      }
    }
    impl AccessPrim for $type {
      #[inline]
      fn access_prim(self, i: usize) -> bool { 
        (self & (1 << i)) != 0 
      }
    }
    impl Select0Prim for $type {
      #[inline]
      fn select0_prim(self, j: usize) -> usize { pdep(1<<j,!self).trailing_zeros() as usize }
    }
    impl Select1Prim for $type {
      #[inline]
      fn select1_prim(self, j: usize) -> usize { pdep(1<<j, self).trailing_zeros() as usize }
    }
    impl Select0 for $type {
      #[inline]
      fn select0(&self, j: usize) -> usize { pdep(1<<j,!*self).trailing_zeros() as usize }
    }
    impl Select1 for $type {
      #[inline]
      fn select1(&self, j: usize) -> usize { pdep(1<<j, *self).trailing_zeros() as usize }
    }
    /// O(n)
    impl Rank for Vec<$type> {
      type Item = bool;
      #[inline]
      fn rank(&self, item: bool, i: usize) -> usize {
        let j = self.rank1(i);
        if item {
          j
        } else {
          i - j
        }
      }
    }
    /// O(n)
    impl BoolRank for Vec<$type> {
      #[inline]
      fn rank1(&self, i: usize) -> usize {
        let (q,r) = div_rem(i,8*size_of::<$type>());
        (0..q).fold(0,|a,b| a + self[b].count_ones() as usize) + self[q].rank1(r)
      }
      #[inline]
      fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
      }
    }

    /// O(1)
    impl Access for Vec<$type> {
      type Item = bool;
      #[inline]
      fn access(&self, i: usize) -> bool {
        let (q,r) = div_rem(i,8*size_of::<$type>());
        self[q].access(r) 
      }
    }

    // TODO: Select[01] for Vec<$type>
  };
}

impl_all!(impl_rank_select: u8, u16, u32, u64);