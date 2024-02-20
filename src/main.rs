pub mod exercises;

//use crate::exercises::print_truth_table::print_truth_table;
use crate::exercises::negation_normal_form::Tree;

// fn test_truth_table() {
//   //print_truth_table("AA!|B&C&");
//   //print_truth_table("BA>");
//   //print_truth_table("BA=");
//   //print_truth_table("BA^");
//   //print_truth_table("AA|!!");
//   //print_truth_table("AB|C|D|E&F^G|");
//   print_truth_table("AB|C|D|E&E^E|");
// }

fn test_tree() {
  let a = Tree::new("ab|cd|&!".chars());
  a.print();
}

fn main() {
  //test_truth_table();
  test_tree();
}