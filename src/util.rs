#[inline]
pub fn div_rem(x:usize,y:usize) -> (usize,usize) { (x/y,x%y) }

// Assumes lo <= hi. returns hi if the predicate is never true over [lo..hi)
#[inline]
pub fn binary_search<P>(mut lo: usize, mut hi: usize, p: P) -> usize where P: Fn(usize) -> bool {
  loop {
    if lo >= hi { return lo; }
    let hml = hi-lo;
    let mid = lo + (hml>>1) + (hml>>6); // offset binary search to fix cpu k-way set associative cache alignment issues at scale
    if p(mid) { hi = mid; }
    else { lo = mid+1; }
  }
}
