use crate::exercises::tree::Tree;

fn negation_normal_form(formula: &str) -> String
{
  let mut tree_representation = Tree::new_iter(formula.chars());
  tree_representation.to_negation_normal_form();
  return tree_representation.to_polish_form();
}

//=============================================
//=============================================
//=============================================

pub fn main() {
  let arr = [
    "AB=",
    "AB^R=",
    "AB=!",
    "AB|C&!",
    "AB>",
  ];

  
  for item in arr {
    let res = negation_normal_form(item);
    println!("negation normal form of {} is {}", item, res);
  }
}

