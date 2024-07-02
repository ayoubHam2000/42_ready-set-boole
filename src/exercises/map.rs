
pub fn test(t : u16, q : u16) -> (u16, u16) {
  let t : usize = t as usize;
  let q : usize = q as usize;

  let index_map: [[u16; 4]; 4] = [
    [0, 1, 2, 3],
    [0, 3, 2, 1],
    [2, 1, 0, 3],
    [2, 3, 0, 1],
  ];

  let next_t_map: [[u16; 4]; 4] = [
      [1, 0, 0, 2],
      [0, 3, 1, 1],
      [2, 2, 3, 0],
      [3, 1, 2, 3],
  ];

  let index = index_map[t][q];
  let next_t = next_t_map[t][q];

  (index, next_t)
}

pub fn get_q(mut x : u16, mut y : u16, size : u16) -> (u16, u16, u16) {
  let q = match (x < size, y < size) {
    (true, false) => {
      y -= size;
      0
    },
    (true, true) => {
      1
    },
    (false, true) => {
      x -= size;
      2
    },
    _ => {
      y -= size;
      x -= size;
      3
    },
  };

  (x, y, q)
}

pub fn map_dim(mut x : u16, mut y : u16, dim : u16) -> u32 {
  let dim : u16 = dim - 1;
  let mut size : u16 = 1 << dim;
  let mut nb: u32 = 1 << (dim * 2);
  let mut p_index : u32 = 0;
  let mut t: u16 = 0;
  
  while nb >= 1 {
    let  q;
    
    (x, y, q) = get_q(x, y, size);
    let (a, next_t) = test(t, q);
    p_index += nb * (a as u32);
    t = next_t;
    nb = nb >> 2;
    size = size >> 1;
  }
  
  p_index
}

pub fn map(x : u16, y : u16) -> f64 {
  // log2(sqrt(2 ^ 16)) = 8
  let nb : u64 = (1 << (8 * 2)) - 1;
  let index = map_dim(x, y , 8);
  return (index as f64) / (nb as f64);
}
