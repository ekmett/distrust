#![allow(dead_code)]
use std::marker::PhantomData;

pub trait Constant { const VALUE : usize; }

pub struct K0;
impl Constant for K0 { const VALUE:usize = 0; }

pub struct K1;
impl Constant for K1 { const VALUE:usize = 1; }

pub struct K2;
impl Constant for K2 { const VALUE:usize = 2; }

pub struct K3;
impl Constant for K3 { const VALUE:usize = 3; }

pub struct B0<K:Constant> { phantom: PhantomData<K> }
impl <K:Constant> Constant for B0<K> { const VALUE:usize = 2*K::VALUE; }

pub struct B1<K:Constant> { phantom: PhantomData<K> }
impl <K:Constant> Constant for B1<K> { const VALUE:usize = 2*K::VALUE+1; }

pub type K4 = B0<K2>;
pub type K5 = B1<K2>;
pub type K6 = B0<K3>;
pub type K7 = B1<K3>;
