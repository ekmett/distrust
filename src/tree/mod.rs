pub mod rc;
pub mod arena;
pub mod jacobson;

// fat binary dag leafy tree
#[derive(Copy,Clone,Debug)]
pub enum View<Id,T> {
  Bin(Id,Id),
  Tip(T)
}

trait Store<T> {
  type Id;
  fn at(&self, i: Self::Id) -> View<Self::Id,T>;
}

trait MutableStore<T> : Store<T> {
  fn tip(&mut self, item: T) -> Self::Id;
  fn bin(&mut self, l: Self::Id, r: Self::Id) -> Self::Id;
}
