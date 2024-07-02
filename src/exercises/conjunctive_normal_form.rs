use crate::exercises::tree::Tree;

fn conjunctive_normal_form(formula: &str) -> String {
  let mut tree_rep = Tree::new_iter(formula.chars());
  tree_rep.to_conjunctive_normal_form();
  return tree_rep.to_polish_form();
}

//======================================
//======================================

pub fn main() {
  let arr = [
    "AB=",
    "AB^R=",
    "AB=!",
    "AB|C&!",
    "AB>",
  ];

  
  for item in arr {
    let res = conjunctive_normal_form(item);
    println!("conjunctive normal form of {} is {}", item, res);
  }
}