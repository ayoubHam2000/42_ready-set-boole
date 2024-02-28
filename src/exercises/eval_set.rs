

fn binary_search(arr : &Vec<i32>, item : i32) -> (bool, usize)
{
  if arr.len() == 0 {
    return (false, 0);
  }

  let mut a = 0;
  let mut b = arr.len() - 1;

  while a <= b {
    let c = (a + b) / 2;

    if arr[c] == item {
      return (true, c);
    }

    if arr[c] < item {
      a = c + 1;
    } else if c > 0 {
      b = c - 1;
    } else {
      break ;
    }
  }

  (false, 0)
}

fn union_set(sets : &Vec<Vec<i32>>) -> Vec<i32> {

  let mut tmp: Vec<i32> = vec![];
  let mut res : Vec<i32> = vec![];

  for set in sets {
    for item in set {
      tmp.push(*item);
    }
  }
  
  if tmp.is_empty() {
    return tmp;
  }

  tmp.sort();
  res.push(tmp[0]);
  let mut last = tmp[0];

  for item in tmp {
    if last != item {
      last = item;
      res.push(last);
    }
  }

  res
}

fn and_set(set_a : &Vec<i32>, set_b : &Vec<i32>) -> Vec<i32> {
  
  if set_a.len() <= set_b.len() {
    let mut res : Vec<i32> = vec![];
    
    for item in set_a {
      if binary_search(&set_b, *item).0 {
        res.push(*item);
      }
    }

    res

  } else {

    and_set(set_b, set_a)

  }

}

fn or_set(set_a : &Vec<i32>, set_b : &Vec<i32>) -> Vec<i32> {
  
  if set_a.len() <= set_b.len() {
    let mut res : Vec<i32> = set_b.clone();

    for item in set_a {
      if binary_search(&set_b, *item).0 == false {
        res.push(*item);
      }
    }

    res

  } else {

    or_set(set_b, set_a)

  }
}

//assuming the global_set is sorted
fn not_set(set : &Vec<i32>, global_set : &Vec<i32>) -> Vec<i32> {
  let mut res = vec![];

  for item in global_set {
    if binary_search(set, *item).0 == false {
      res.push(*item);
    }
  }

  res
}

pub enum Token {
  Negation,
  Conjunction,
  Disjunction,
  ExclusiveDisjunction,
  MaterialCondition,
  LogicalEquivalence,
  Value(char),
}

impl Token {
  pub fn new(c : char) -> Token {
    match c {
      '!' => Token::Negation,
      '&' => Token::Conjunction,
      '|' => Token::Disjunction,
      '^' => Token::ExclusiveDisjunction,
      '>' => Token::MaterialCondition,
      '=' => Token::LogicalEquivalence,
      _ => {
        if c.is_alphabetic() && c.is_uppercase() {
          Token::Value(c)
        } else {
          panic!("invalid token found {}", c);
        }
      }
    }
  }
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
  //todo basic_operation_formula
  let mut stack : Vec<Vec<i32>>  = Vec::new();
  let global_set = union_set(&sets);
  
  for c in formula.chars() {
    let token = Token::new(c);
    
    let mut pop_stack = || {
      stack.pop().expect("Expected a set but found the stack empty!")
    };

    match token {
      Token::Negation => {
        let v1  = pop_stack();
        stack.push(not_set(&v1, &global_set));
      }, 
      Token::Conjunction => {
        let v1 = pop_stack();
        let v2 = pop_stack();
        stack.push(and_set(&v1, &v2));
      }, 
      Token::Disjunction => {
        let v1 = pop_stack();
        let v2 = pop_stack();
        stack.push(or_set(&v1, &v2));
      },
      Token::ExclusiveDisjunction => {
        //a ^ b = (a and !b) | (!a and b)
        let a = pop_stack();
        let b = pop_stack();
        let not_a = not_set(&a, &global_set);
        let not_b = not_set(&b, &global_set);
        let a = and_set(&a, &not_b);
        let b = and_set(&b, &not_a);
        stack.push(or_set(&a, &b));
      }, 
      Token::MaterialCondition => {
        //a > b = (!a | b)
        let a = pop_stack();
        let b = pop_stack();
        let not_a = not_set(&a, &global_set);
        stack.push(or_set(&not_a, &b));
      }, 
      Token::LogicalEquivalence => {
        //a and b = (a and b) | (!a and !b)
        let a = pop_stack();
        let b = pop_stack();
        let not_a = not_set(&a, &global_set);
        let not_b = not_set(&b, &global_set);
        let a = and_set(&a, &b);
        let b = and_set(&not_a, &not_b);
        stack.push(or_set(&a, &b));
      },
      Token::Value(a) => {
        let index = ((a as i32) - ('A' as i32)) as usize;
        if index > sets.len() {
          panic!("the set {} does not exist", a);
        }
        let mut new_set = sets[index].clone();
        new_set.sort();
        stack.push(new_set);
      }
    }

  }
  
  if stack.len() == 1 {
    return stack.pop().unwrap();
  }
  
  panic!("Invalid expression");
}

