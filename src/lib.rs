pub mod exercises;



#[cfg(test)]
mod tests {


  use crate::exercises::{
    adder::adder,
    multiplier::multiplier,
    gray_code::gray_code,
    eval_formula::eval_formula,
    print_truth_table::print_truth_table
  };
  

  #[test]
  fn test_adder() {
    assert_eq!(adder(10, 5), 15);
    assert_eq!(adder(7, 7), 14);
    assert_eq!(adder(759, 255), 1014);
    assert_eq!(adder(352, 74), 426);
    assert_eq!(adder(0xfffffffe, 1), 0xffffffff);
  }

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

  #[test]
  fn test_gray_code() {
    assert_eq!(gray_code(0), 0);
    assert_eq!(gray_code(1), 1);
    assert_eq!(gray_code(2), 3);
    assert_eq!(gray_code(6985), 5869);
    assert_eq!(gray_code(8), 12);
  }

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


  }

  #[test]
  fn test_eval_formula_panic() {
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

  #[test]
  fn test_print_truth_table() {

  }

}
