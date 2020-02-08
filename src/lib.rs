extern crate bitintr;
use bitintr::x86::bmi2::{pdep,bzhi};

pub fn select(x: u64, j: u32) -> u32 {
  pdep(1u64 << j, x).trailing_zeros()
}

pub fn rank(x: u64, j: u32) -> u32 {
  bzhi(x,j as u64).count_ones()
}
