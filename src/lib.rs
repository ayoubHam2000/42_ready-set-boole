pub mod exercises;

// cargo test [name of the test]

#[cfg(test)]
mod tests {


  use crate::exercises::{
    adder::adder,
    multiplier::multiplier,
    gray_code::gray_code,
    eval_formula::eval_formula,
    print_truth_table::print_truth_table,
    negation_normal_form,
    conjunctive_normal_form,
    sat::sat,
    powerset::powerset,
    eval_set::eval_set,
    map::map,
    reverse_map::reverse_map,
  };
  

  //========================================
  //======= EX00 ===========================
  //========================================
  #[test]
  fn test_adder() {
    assert_eq!(adder(10, 5), 15);
    assert_eq!(adder(7, 7), 14);
    assert_eq!(adder(759, 255), 1014);
    assert_eq!(adder(352, 74), 426);
    assert_eq!(adder(0xfffffffe, 1), 0xffffffff);
  }

  //========================================
  //======= EX01 ===========================
  //========================================
  #[test]
  fn test_multiplier() {
    assert_eq!(multiplier(70, 7), 490);
    assert_eq!(multiplier(7, 70), 490);
    assert_eq!(multiplier(7, 7), 49);
    assert_eq!(multiplier(0, 0), 0);
    assert_eq!(multiplier(0, 1), 0);
    assert_eq!(multiplier(1, 0), 0);
    assert_eq!(multiplier(8569, 45687), 8569 * 45687);
  }

  //========================================
  //======= EX02 ===========================
  //========================================
  #[test]
  fn test_gray_code() {
    assert_eq!(gray_code(0), 0);
    assert_eq!(gray_code(1), 1);
    assert_eq!(gray_code(2), 3);
    assert_eq!(gray_code(6985), 5869);
    assert_eq!(gray_code(8), 12);
  }


  //========================================
  //======= EX03 ===========================
  //========================================
  #[test]
  fn test_eval_formula() {
    assert_eq!(eval_formula("10&"), false);
    assert_eq!(eval_formula("10|"), true);
    assert_eq!(eval_formula("10|1&"), true);
    assert_eq!(eval_formula("01|01|&"), true);
    assert_eq!(eval_formula("1!0|"), false);
    assert_eq!(eval_formula("1!0|"), false);
    assert_eq!(eval_formula("0"), false);

    assert_eq!(eval_formula("10&"), false);
    assert_eq!(eval_formula("10|"), true);
    assert_eq!(eval_formula("11>"), true);
    assert_eq!(eval_formula("10="), false);
    assert_eq!(eval_formula("1011||="), true);


    //Should panic
    //Should panic


    {
      let result = std::panic::catch_unwind(|| {
        eval_formula("5");
      });
      assert!(result.is_err());
    }

    {
      let result = std::panic::catch_unwind(|| {
        eval_formula("1~");
      });
      assert!(result.is_err());
    }

    {
      let result = std::panic::catch_unwind(|| {
        eval_formula("11||");
      });
      assert!(result.is_err());
    }

    {
      let result = std::panic::catch_unwind(|| {
        eval_formula("11");
      });
      assert!(result.is_err());
    }

  }


  //========================================
  //======= EX04 ===========================
  //========================================
  // cargo test test_print_truth_table -- --nocapture
  #[test]
  fn test_print_truth_table() {

    print_truth_table("AB|");
    print_truth_table("ABC^|");
    print_truth_table("AB=ER=&!R|");


    //Should panic
    //Should panic
    {
      let result = std::panic::catch_unwind(|| {
        print_truth_table("5");
      });
      assert!(result.is_err());
    }

    {
      let result = std::panic::catch_unwind(|| {
        print_truth_table("AB@");
      });
      assert!(result.is_err());
    }

    {
      let result = std::panic::catch_unwind(|| {
        print_truth_table("");
      });
      assert!(result.is_err());
    }

    {
      let result = std::panic::catch_unwind(|| {
        print_truth_table("|");
      });
      assert!(result.is_err());
    }

  }


  //========================================
  //======= EX05 ===========================
  //========================================
  // cargo test test_print_truth_table -- --nocapture
  #[test]
  fn test_negation_normal_form() {
    negation_normal_form::main();
  }


  //========================================
  //======= EX06 ===========================
  //========================================
  // cargo test test_print_truth_table -- --nocapture
  #[test]
  fn test_conjunctive_normal_form() {
    conjunctive_normal_form::main();
  }


  //========================================
  //======= EX07 ===========================
  //========================================
  // cargo test test_print_truth_table -- --nocapture
  #[test]
  fn test_sat() {
    assert_eq!(sat("AB|"), true);
    assert_eq!(sat("AA!&"), false);
    assert_eq!(sat("AB="), true);
    assert_eq!(sat("AB|B!&A!&"), false);
    assert_eq!(sat("AA^"), false);
  }

  //========================================
  //======= EX08 ===========================
  //========================================
  #[test]
  fn test_powerset() {
    let res = powerset(vec![2, 4]);

    for item in res {
      println!("=> {:?}", item);
    }
  }

  //========================================
  //======= EX09 ===========================
  //========================================
  #[test]
  fn test_eval_set() {
    let res = eval_set("A!B&", vec![
      vec![1, 3, 5],
      vec![2, 1],
    ]);

    println!("=> {:?}", res);
  }


  //========================================
  //======= EX10 And EX11 ==================
  //========================================

  #[test]
  fn test_map_reverse_map() {
    for x in 0..=255 {
      for y in 0..=255 {
        println!("{} {}", x, y);
        assert_eq!(reverse_map(map(x, y)), (x, y));
      }
    }
  }



}
