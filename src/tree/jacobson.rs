use crate::succinct::*;
use crate::tree::*;

#[derive(Clone,Debug)]
struct Jacobson<K,T> {
  pub index: K,
  pub data: Vec<T>
}

impl <K:BoolRank+Access<Item=bool>,T:Clone> Store<T> for Jacobson<K,T> {
  type Id = usize;
  fn at(&self, i: Self::Id) -> View<Self::Id,T> {
    if self.index.access(i) {
      let j = self.index.rank1(2*i);
      View::Bin(j,j+1)
    } else {
      View::Tip(self.data[i-self.index.rank1(i)].clone())
    }
  }
}