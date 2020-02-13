use crate::succinct::*;

pub trait Dim: Copy {
  fn dim(self) -> usize;
}

/// dynamic size
impl Dim for usize {
  fn dim(self) -> usize { self }
}

/// A `k^d` tree is a generalized Jacobson tree encoding for a 2^d-radix tree
#[derive(Copy,Clone,Debug)]
pub struct Tree<K,D> {
  pub index: K,
  pub arity: D
}

impl <K,D> Tree<K,D> {
  pub fn root(self) -> Zipper<K,D> { Zipper(self,1) }
  // fn at(self, i: usize) -> Zipper<K,D> { Zipper(self,i) }
}

#[allow(non_snake_case)]
pub fn Tree<K,D>(index: K, arity: D) -> Tree<K,D> {
  Tree{index,arity}
}

impl <K:Copy,D:Dim> Dim for Tree<K,D> {
  fn dim(self) -> usize { self.arity.dim() }
}

impl <K:Access,D> Access for Tree<K,D> {
  #[inline]
  fn access(self,i: usize) -> bool {
    self.index.access(i)
  }
}

impl <K:Rank,D> Rank for Tree<K,D> {
  #[inline]
  fn rank(self,i: usize) -> usize {
    self.index.rank(i)
  }
}

impl <K:Select0,D> Select0 for Tree<K,D> {
  #[inline]
  fn select0(self,i: usize) -> usize {
    self.index.select0(i)
  }
}

impl <K:Select1,D> Select1 for Tree<K,D> {
  #[inline]
  fn select1(self,i: usize) -> usize {
    self.index.select1(i)
  }
}

// children

/// points to the left-most of a run of siblings
#[derive(Copy,Clone,Debug)]
pub struct Children<K,D> {
  pub tree: Tree<K,D>,
  pub cursor: usize
}

impl <K:Copy,D:Dim> Dim for Children<K,D> {
  #[inline]
  fn dim(self) -> usize { self.tree.arity.dim() }
}

impl <K:Copy,D:Dim> Children<K,D> {
  #[inline]
  pub fn root(self) -> Zipper<K,D> { self.at(1) }
  #[inline]
  fn at(self, i: usize) -> Zipper<K,D> { Zipper(self.tree,i) }
  #[inline]
  /// O(select1)
  pub fn parent(self) -> Zipper<K,D> where K : Select1 + Copy {
    self.at(self.tree.index.select1(self.cursor/self.dim()))
  }
  #[inline]
  pub fn nth(&self, k: usize) -> Zipper<K,D> {
    assert!(k <= self.dim());
    self.at(self.cursor+k)
  }
}

/// this is a zipper into a tree
#[derive(Copy,Clone,Debug)]
pub struct Zipper<K,D> {
  pub tree: Tree<K,D>,
  pub cursor: usize
}

// local faux positional constructor
#[allow(non_snake_case)]
fn Zipper<K,D>(tree: Tree<K,D>, cursor: usize) -> Zipper<K,D> {
  Zipper{tree,cursor}
}

impl <K:Copy,D:Dim> Dim for Zipper<K,D> {
  fn dim(self) -> usize { self.tree.arity.dim() }
}

impl <K:Copy,D:Dim> Zipper<K,D> {
  pub fn root(self) -> Zipper<K,D> { Zipper(self.tree,1) }
  fn at(self, i: usize) -> Zipper<K,D> { Zipper(self.tree,i) }
  #[inline]
  ///O(1)
  pub fn top(&self) -> bool { self.cursor == 1 }
  #[inline]
  ///O(access)
  pub fn tip(&self) -> bool where K : Access {
    !self.tree.index.access(self.cursor)
  }
  #[inline]
  /// O(access+rank)
  pub fn children(&self) -> Option<Children<K,D>> where K : Rank + Access {
    if self.tree.index.access(self.cursor) {
      Some(Children {
        tree: self.tree,
        cursor: self.tree.index.rank(2*self.cursor)
      })
    } else {
      None
    }
  }
  #[inline]
  /// O(select1)
  pub fn parent(&self) -> Option<Zipper<K,D>> where K : Select1 {
    if self.top() {
      None
    } else {
      Some(self.at(self.tree.index.select1(self.cursor/self.dim())))
    }
  }
  // s.unsafe_child(k) -- assumes !s.tip(), 0 <= k < self.dim()
  #[inline]
  /// O(rank)
  pub unsafe fn unsafe_child(&self, k: usize) -> Zipper<K,D> where K : Rank {
    self.at(self.tree.index.rank(2*self.cursor) + k)
  }
  // assumes !self.top()
  #[inline]
  /// O(select1)
  pub unsafe fn unsafe_parent(&self) -> Zipper<K,D> where K : Select1 {
    self.at(self.tree.index.select1(self.cursor/self.tree.arity.dim()))
  }
  #[inline]
  /// O(access+rank), 0 <= k < self.dim()
  pub fn child(&self, k: usize) -> Option<Zipper<K,D>> where K : Rank + Access {
    Some(self.children()?.nth(k))
  }
}
