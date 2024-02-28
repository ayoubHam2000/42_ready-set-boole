pub fn powerset(set : Vec<i32>) -> Vec<Vec<i32>> {
  let mut res : Vec<Vec<i32>> = vec![];

  for item in set {
    let mut tmp : Vec<Vec<i32>> = vec![];

    for sub_set in &res {
      let mut new_sub_set = sub_set.clone();
      new_sub_set.push(item);
      tmp.push(new_sub_set);
    }
    for sub_set in tmp {
      res.push(sub_set);
    }

    let sub_set = vec![item];
    res.push(sub_set);
  }
  res.push(vec![]);

  res
}