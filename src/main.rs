

pub mod exercises;

// use crate::exercises::print_truth_table::print_truth_table;

// fn test_truth_table() {
//   //print_truth_table("AA!|B&C&");
//   print_truth_table("abcd==|");
//   print_truth_table("bb!|a|c!d!|b|a|cd|b|a|&&cc!|cd!|&dc!|&dd!|&b!|a|&c!d!|c|c!d!|d|&cd|c|&c!|a|c!d!|c|c!d!|d|&cd|c|&d!|a|&cd|d|c!|a|&cd|d|d!|a|&&");
//   //print_truth_table("BA=");
//   //print_truth_table("BA^");
//   //print_truth_table("AA|!!");
//   //print_truth_table("AB|C|D|E&F^G|");
//   //print_truth_table("AB|C|B|E&E^E|");
// }

// fn main() {
//   test_truth_table();
// }


// use crate::exercises::sat::sat;
// use crate::exercises::print_truth_table::print_truth_table;
// use crate::exercises::negation_normal_form::Tree;


// fn main() {
//   //(A ∨ B ∨ C) ∧ (¬A ∨ ¬B ∨ ¬C) ∧ (A ∨ D) ∧ (¬A ∨ ¬D) ∧ (B ∨ E) ∧ (¬B ∨ ¬E)
//   let res = sat("abcde||||a!&b!&c!&d!&e!&");
//   print_truth_table("abcde||||a!&b!&c!&d!&e!&");
//   let t = Tree::new_iter("abcde||||a!&b!&c!&d!&e!&".chars());

//   t.print_as_string();
//   println!("{}", res);
// }

// use crate::exercises::negation_normal_form::Tree;
// fn test_tree() {

//   let mut a = Tree::new_iter("abcd==|".chars());
//   //let mut a = Tree::new_iter("ac|".chars());
//   //a.print();
//   //a.to_basic_logic_operators();
//   //a.to_negation_normal_form();
//   a.print_as_string();
//   a.to_conjunctive_normal_form();
//   //a.print();
//   a.print_as_string();
//   let polish_form = a.to_polish_form();
//   println!("{}", polish_form);



// }

// fn main() {
//   test_tree();
// }

// use exercises::powerset::powerset;

// fn main() {
//   let res = powerset(vec![1, 2, 3, 4, 5, 6, 7]);

//   println!("{}", res.len());
//   for item in res {
//     println!("{:?}", item);
//   }
// }


use exercises::eval_set::eval_set;

fn main() {
  let mut sets: Vec<Vec<i32>> = vec![];

  sets.push(vec![1, 2, 3, 0]);
  sets.push(vec![3, 9, 5, 1, 8]);
  let res = eval_set("AB>", sets);

  println!("{:?}", res);
}