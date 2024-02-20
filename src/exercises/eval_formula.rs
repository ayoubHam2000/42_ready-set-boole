pub struct Stack<T> {
  items : Vec<T>
}

impl<T> Stack<T> {
  pub fn new() -> Stack<T> {
    Stack { items : Vec::new() }
  }

  pub fn empty(&self) -> bool {
    self.items.is_empty()
  }

  pub fn push(& mut self, value : T) {
    self.items.push(value);
  }

  pub fn pop(& mut self) -> Option<T> {
    self.items.pop()
  }

  pub fn size(&self) -> usize {
    self.items.len()
  }

}

pub enum Operators {
  Negation,
  Conjunction,
  Disjunction,
  ExclusiveDisjunction,
  MaterialCondition,
  LogicalEquivalence,
  Value(bool),
  None
}

impl Operators {
  pub fn new(c : char) -> Operators {
    match c {
      '!' => Operators::Negation,
      '&' => Operators::Conjunction,
      '|' => Operators::Disjunction,
      '^' => Operators::ExclusiveDisjunction,
      '>' => Operators::MaterialCondition,
      '=' => Operators::LogicalEquivalence,
      '0' => Operators::Value(false),
      '1' => Operators::Value(true),
      _ => Operators::None,
    }
  }
}

pub fn eval_formula_iter<I : Iterator<Item = char>>(formula : I) -> bool {
  let mut stack : Stack<bool>  = Stack::new();
  
  for c in formula {
    let token = Operators::new(c);
    let mut pop_stack = || {
      stack.pop().expect("Expected a value but found the stack empty!")
    };

    match token {
      Operators::Negation => {
        let v1  = pop_stack();
        stack.push(!v1);
      }, 
      Operators::Conjunction => {
        let v1 = pop_stack();
        let v2 = pop_stack();
        stack.push(v1 & v2);
      }, 
      Operators::Disjunction => {
        let v1 = pop_stack();
        let v2 = pop_stack();
        stack.push(v1 | v2);
      }, 
      Operators::ExclusiveDisjunction => {
        let v1 = pop_stack();
        let v2 = pop_stack();
        stack.push(v1 ^ v2);
      }, 
      Operators::MaterialCondition => {
        let v1 = pop_stack();
        let v2 = pop_stack();
        stack.push(!v1 | v2);
      }, 
      Operators::LogicalEquivalence => {
        let v1 = pop_stack();
        let v2 = pop_stack();
        stack.push((v1 & v2) | (!v1 & !v2));
      },
      Operators::Value(a) => stack.push(a),
      Operators::None => panic!("Invalid operator")
    }

  }
  
  if stack.size() == 1 {
    return stack.pop().unwrap();
  }

  panic!("Invalid expression")
}

pub fn eval_formula(formula : &str) -> bool {
  eval_formula_iter(formula.chars())
}
