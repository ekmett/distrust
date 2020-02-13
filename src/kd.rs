use std::marker::PhantomData;
use crate::succinct::*;
use crate::constant::*;

/// `k^d` tree
pub struct KD<D:Constant> { index : Poppy, phantom : PhantomData<D> }
