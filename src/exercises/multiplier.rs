
use crate::exercises::adder::adder;

pub fn multiplier(a : u32, b : u32) -> u32 {
  let mut res ;

  let mut a = a;
  let mut b = b;

  res = 0;
  while b != 0 {
    if b & 1 == 1 {
      res = adder(res, a);
    }
    a = a << 1;
    b = b >> 1;
  }

  res
}



