use std::rc::Rc;

/// [Optimal Joins using Compact Data Structures](https://arxiv.org/pdf/1908.01812.pdf)

// we need data associated with the leaves, too
type Q = Zipper<Poppy,usize>
type Shuffle = u32 // actually should allow for reorderings too

pub enum LQDAG {
  Empty,
  QTree(Rc<Q>),
  NotQTree(Rc<Q>),
  And(Rc<LQDAG>,Rc<LQDAG>),
  Or(Rc<LQDAG>,Rc<LQDAG>),
  Extend(Rc<LQDAG>,Shuffle) // give me a  shuffle of the fields
}

pub enum Value {
  Zero,
  One,
  Half, 
  Dia
}


