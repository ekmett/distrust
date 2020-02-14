use std::rc::Rc;
use crate::tree::*;

#[derive(Clone,Debug)]
struct RcTree<T>(Rc<View<RcTree<T>,T>>);
struct RcStore;

impl <T:Clone> Store<T> for RcStore {
  type Id = RcTree<T>;
  fn at(&self, i: RcTree<T>) -> View<RcTree<T>,T> {
    i.0.as_ref().clone()
  }
}
impl <T:Clone> MutableStore<T> for RcStore {
  fn tip(&mut self, item: T) -> RcTree<T> { 
    RcTree(Rc::new(View::Tip(item)))
  }
  fn bin(&mut self, l: RcTree<T>, r: RcTree<T>) -> RcTree<T> {
    RcTree(Rc::new(View::Bin(l,r)))
  }  
}