
pub fn adder(a: u32, b: u32) -> u32 {
  let mut sum;
  let mut cary = 0;
  let mut res = 0;

  for i in 0..32 {
    let x = (a >> i) & 1;
    let y = (b >> i) & 1;
    sum = (x) ^ (y) ^ (cary);
    cary = (cary & (x | y)) | (x & y);
    res = res | (sum << i);
  }

  res
}


