
use std::collections::HashSet;

use crate::exercises::eval_formula::eval_formula;

fn is_operator(item : &char) -> bool {
  "!&|^>=".contains(*item)
}

fn get_input(formula : &str, bits : u32, variables : &Vec<char>) -> String {
  let mut i = variables.len() - 1;
  let mut result = formula.to_string();

  for var in variables {
    let bit_value = (bits >> i) & 1;
  
    let b = if bit_value == 1 {
      "1"
    } else {
      "0"
    };
    
    result = result.replace(*var, b);

    if i > 0 {
      i -= 1;
    }
  }

  return result;
}


pub fn sat(formula: &str) -> bool {
  let mut variables : Vec<char> = formula.chars()
      .filter(|&c| !is_operator(&c))
      .collect::<HashSet<char>>().into_iter().collect();
  variables.sort();
  let is_valid_input = formula.chars().all(|c| c.is_alphabetic() | is_operator(&c));
  let is_at_least_one_variable = variables.len() > 0;
  if !is_valid_input {
    panic!("Invalid input");
  }
  if !is_at_least_one_variable {
    panic!("It should be at least one variable in the input");
  }
  
  let mut bits = 0;
  let max = 2 << (variables.len() - 1);

  while bits < max {
    let formula = get_input(formula, bits, &variables);
    if eval_formula(&formula) {
      return true;
    }
    bits += 1;
  }
  
  false
}
