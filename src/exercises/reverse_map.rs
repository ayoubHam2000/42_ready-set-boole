
pub fn get_q(n : f64) -> (f64, u16) {
  let mut n = n;

  let q = match (n < 0.25, n < 0.5, n < 0.75, n < 1.0) {
    (true, true, true, true) => {
      n = n / (0.25);
      0
    },
    (false, true, true, true) => {
      n = (n - 0.25) / (0.25);
      1
    },
    (false, false, true, true) => {
      n = (n - 0.5) / (0.25);
      2
    },
    _ => {
      n = (n - 0.75) / (0.25);
      3
    },
  };

  (n, q)
}

pub fn get_xy(q : u16, t : u16) -> (u16, u16, u16) {
  let t : usize = t as usize;
  let q : usize = q as usize;

  let arr_x: [[u16; 4]; 4] = [
    [0, 0, 1, 1],
    [0, 1, 1, 0],
    [1, 0, 0, 1],
    [1, 1, 0, 0],
  ];
  
  let arr_y: [[u16; 4]; 4] = [
      [1, 0, 0, 1],
      [1, 1, 0, 0],
      [0, 0, 1, 1],
      [0, 1, 1, 0],
  ];

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


  let a = index_map[t][q] as usize;
  (next_t_map[t][a], arr_x[t][q], arr_y[t][q])
}

pub fn reverse_map_dim(n : f64, dim : u16) -> (u16, u16) {
  let mut n = n;
  let mut x = 0;
  let mut y = 0;
  let mut size : u16 = 1 << (dim - 1);
  let mut t: u16 = 0;
  
  while size >= 1 {
    let q;

    (n, q) = get_q(n);
    let (next_t, add_x, add_y) = get_xy(q, t);
    t = next_t;
    x += add_x * size;
    y += add_y * size;
    //println!("{} {} => {} {} t={} q={}", n, n * ((1 << (dim *(2))) as f64), x, y, t, q);
    size = size >> 1;
  }

  (x, y)
}

pub fn reverse_map(n : f64) -> (u16, u16) {
  return reverse_map_dim(n, 8);
}