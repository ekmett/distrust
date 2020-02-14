use crate::succinct::*;
use crate::tree::*;

#[derive(Clone,Debug)]
struct Jacobson<K,D> {
  pub index: K, // index holding the jacobson tree structure
  pub data: D // array or vector or other structure holding the data per element
}

impl <K:BoolRank+Access<Item=bool>,D:Access> Store<D::Item> for Jacobson<K,D> where D::Item : Clone {
  type Id = usize;
  fn at(&self, i: usize) -> View<usize,D::Item> {
    if self.index.access(i) {
      let j = self.index.rank1(i+i);
      View::Bin(j,j+1)
    } else {
      View::Tip(self.data.access(self.index.rank0(i)))
    }
  }
}