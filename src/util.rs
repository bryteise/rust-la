use std::vec;
use std::num;
use std::num::{Zero, One};

#[inline]
pub fn alloc_dirty_vec<T>(n : uint) -> ~[T] {
  let mut v : ~[T] = vec::with_capacity(n);
  unsafe {
    vec::raw::set_len(&mut v, n);
  }
  v
}

// Ported from JAMA.
// sqrt(a^2 + b^2) without under/overflow.
pub fn hypot<T : Zero + One + Signed + Algebraic + Orderable + Clone>(a : T, b : T) -> T {
  if num::abs(a.clone()) > num::abs(b.clone()) {
    let r = b / a;
    return num::abs(a.clone()) * num::sqrt(One::one::<T>() + r * r);
  } else if b != Zero::zero() {
    let r = a / b;
    return num::abs(b.clone()) * num::sqrt(One::one::<T>() + r * r);
  } else {
    return Zero::zero();
  }
}
