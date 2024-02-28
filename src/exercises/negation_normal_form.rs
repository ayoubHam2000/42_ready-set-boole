use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
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
      other => Token::Value(other),
    }
  }

  pub fn get_value(&self) -> char {
    match self {
      Token::Negation => '!',
      Token::Conjunction => '&',
      Token::Disjunction => '|',
      Token::ExclusiveDisjunction => '^',
      Token::MaterialCondition => '>',
      Token::LogicalEquivalence => '=',
      Token::Value(a) => *a,
    }
  }
}

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



pub struct Node {
  value : Token,
  left : Option<Rc<RefCell<Node>>>,
  right : Option<Rc<RefCell<Node>>>
}



impl Node {
  pub fn new(t : Token, left : Option<Rc<RefCell<Node>>>, right : Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {

  
    let new_node = Node {
      value : t,
      left : left,
      right : right
    };

    Rc::new(RefCell::new(new_node))
  }

  pub fn get_height(&self) -> usize {
    let mut a = 0;
    let mut b = 0;

    if let Some(left) = &self.left {
      a = left.borrow().get_height();
    }
    if let Some(right) = &self.right {
      b = right.borrow().get_height();
    }
    
    let max = if a > b {
      a
    } else {
      b
    };

    return max + 1;
  }

  pub fn to_basic_logic_operators(&self) -> Rc<RefCell<Node>> {
    // let new_tree = Node::new()

  
    let node = match self.value {
      Token::ExclusiveDisjunction => {
        //a xor b --> a!b or !ab
        let a = self.left.as_ref().unwrap().borrow().to_basic_logic_operators();
        let not_a = self.left.as_ref().unwrap().borrow().to_basic_logic_operators();
        let b = self.right.as_ref().unwrap().borrow().to_basic_logic_operators();
        let not_b = self.right.as_ref().unwrap().borrow().to_basic_logic_operators();

        let not_b = Node::new(Token::Negation, Some(not_b), None);
        let not_a = Node::new(Token::Negation, Some(not_a), None);
        let conjunction_node_1 = Node::new(Token::Conjunction, Some(a), Some(not_b));
        let conjunction_node_2 = Node::new(Token::Conjunction, Some(not_a), Some(b));
        
        Node::new(Token::Disjunction, Some(conjunction_node_1), Some(conjunction_node_2))
      },
      Token::MaterialCondition => {
        //a => b --> !a or b
        let not_a = self.left.as_ref().unwrap().borrow().to_basic_logic_operators();
        let b = self.right.as_ref().unwrap().borrow().to_basic_logic_operators();

        let not_a = Node::new(Token::Negation, Some(not_a), None);

        Node::new(Token::Disjunction, Some(not_a), Some(b))
      }
      Token::LogicalEquivalence => {
        // a <=> b --> (a and b) or (!a and !b)
        let a = self.left.as_ref().unwrap().borrow().to_basic_logic_operators();
        let not_a = self.left.as_ref().unwrap().borrow().to_basic_logic_operators();
        let b = self.right.as_ref().unwrap().borrow().to_basic_logic_operators();
        let not_b = self.right.as_ref().unwrap().borrow().to_basic_logic_operators();

        let not_b = Node::new(Token::Negation, Some(not_b), None);
        let not_a = Node::new(Token::Negation, Some(not_a), None);

        let conjunction_node_1 = Node::new(Token::Conjunction, Some(a), Some(b));
        let conjunction_node_2 = Node::new(Token::Conjunction, Some(not_a), Some(not_b));

        Node::new(Token::Disjunction, Some(conjunction_node_1), Some(conjunction_node_2))
      },
      Token::Conjunction |
      Token::Disjunction => {
        let a = self.left.as_ref().unwrap().borrow().to_basic_logic_operators();
        let b = self.right.as_ref().unwrap().borrow().to_basic_logic_operators();
        
        Node::new(self.value.clone(), Some(a), Some(b))
      },
      Token::Negation => {
        let a = self.left.as_ref().unwrap().borrow().to_basic_logic_operators();
        
        Node::new(self.value.clone(), Some(a), None)
      }
      Token::Value(c) => {
        Node::new(Token::new(c), None, None)
      }
    };

    node
    
  }

  //the input should be in basic logic operators
  pub fn to_negation_normal_form(&self) -> Rc<RefCell<Node>> {
    //println!("counter");
    match self.value {
      
      Token::Negation => {
        let a = self.left.as_ref().unwrap().borrow();
        
        match a.value {
          Token::Conjunction |
          Token::Disjunction => {
            let left = a.left.as_ref().unwrap().borrow().to_negation_normal_form();
            let right = a.right.as_ref().unwrap().borrow().to_negation_normal_form();
            let not_left = Node::new(Token::Negation, Some(left), None);
            let not_right = Node::new(Token::Negation, Some(right), None);
            let not_left = not_left.borrow().to_negation_normal_form();
            let not_right = not_right.borrow().to_negation_normal_form();
            
            if a.value == Token::Conjunction {
              Node::new(Token::Disjunction, Some(not_left), Some(not_right))
            } else {
              Node::new(Token::Conjunction, Some(not_left), Some(not_right))
            }
          },
          Token::Negation => {
            a.left.as_ref().unwrap().borrow().to_negation_normal_form()
          },
          Token::Value(_) => {
            let left = Node::new(a.value.clone(), None, None);
            Node::new(Token::Negation, Some(left), None)
          },
          _ => {
            panic!("the expression must be in negation normal form.")
          }
        }

      },

      Token::Conjunction |
      Token::Disjunction => {
        let a = self.left.as_ref().unwrap().borrow().to_negation_normal_form();
        let b = self.right.as_ref().unwrap().borrow().to_negation_normal_form();
        Node::new(self.value.clone(), Some(a), Some(b))
      }

      Token::Value(_) => {
        Node::new(self.value.clone(), None, None)
      },
      _ => {
        panic!("the expression must be in negation normal form.")
      }
    }

    
  }

  //the self should be in negation normal form
  pub fn to_conjunctive_normal_form(&self) -> Rc<RefCell<Node>> {
    
    match self.value {
      Token::Disjunction => {
        let left_ref = self.left.as_ref().unwrap();
        let right_ref = self.right.as_ref().unwrap();
        
        let left = left_ref.borrow();
        let right = right_ref.borrow();
        
        //println!("{} {} {} ", self.value.get_value(), left.value.get_value(), right.value.get_value());

        if let (Token::Conjunction, Token::Conjunction) = (&left.value, &right.value)
        {
          //(a & b) or (c & d) => (a or c) and (a or d) and (b or c) and (b or d)
          let a = left.left.as_ref().unwrap();
          let b = left.right.as_ref().unwrap();
          let c = right.left.as_ref().unwrap();
          let d = right.right.as_ref().unwrap();
          
          let disj_a = Node::new(Token::Disjunction, Some(a.clone()), Some(c.clone()));
          let disj_b = Node::new(Token::Disjunction, Some(a.clone()), Some(d.clone()));
          let disj_c = Node::new(Token::Disjunction, Some(b.clone()), Some(c.clone()));
          let disj_d = Node::new(Token::Disjunction, Some(b.clone()), Some(d.clone()));
          
          let disj_a = disj_a.borrow().to_conjunctive_normal_form();
          let disj_b = disj_b.borrow().to_conjunctive_normal_form();
          let disj_c = disj_c.borrow().to_conjunctive_normal_form();
          let disj_d = disj_d.borrow().to_conjunctive_normal_form();

          let conj_1 = Node::new(Token::Conjunction, Some(disj_a), Some(disj_b));
          let conj_2 = Node::new(Token::Conjunction, Some(conj_1), Some(disj_c));
          
          Node::new(Token::Conjunction, Some(conj_2), Some(disj_d))
        }

        else if let (Token::Disjunction, Token::Disjunction) = (&left.value, &right.value) 
        {
          let left = left.to_conjunctive_normal_form();
          let right = right.to_conjunctive_normal_form();
          let new_node = Node::new(Token::Disjunction, Some(left), Some(right));
          
          let a = new_node.borrow().left.as_ref().unwrap().borrow().value.clone();
          let b = new_node.borrow().right.as_ref().unwrap().borrow().value.clone();

          if let (Token::Disjunction, Token::Disjunction) = (a, b) {
            new_node
          } else {
            let res = new_node.borrow().to_conjunctive_normal_form();
            res
          }
        } 

        else if let (Token::Conjunction, Token::Disjunction) = (&left.value, &right.value)
        {
          // (a and b) or (c or d) => ((a and b) or c) or d
          let a = left.left.as_ref().unwrap();
          let b = left.right.as_ref().unwrap();
          let c = right.left.as_ref().unwrap();
          let d = right.right.as_ref().unwrap();

     

          let conj = Node::new(Token::Conjunction, Some(a.clone()), Some(b.clone()));
          let disj = Node::new(Token::Disjunction, Some(conj), Some(c.clone()));
          
          let disj = disj.borrow().to_conjunctive_normal_form();
          let disj = Node::new(Token::Disjunction, Some(disj), Some(d.clone()));

          let res = disj.borrow().to_conjunctive_normal_form();
          res
        }

        else if let (Token::Disjunction, Token::Value(_)) = (&left.value, &right.value)
        {
          let a = left.to_conjunctive_normal_form();

          let disj = Node::new(Token::Disjunction, Some(a), Some(right_ref.clone()));

          let left = disj.borrow().left.as_ref().unwrap().borrow().value.clone();
          let right = disj.borrow().right.as_ref().unwrap().borrow().value.clone();
          
          if let (Token::Disjunction, Token::Value(_)) = (left, right) {
            disj
          } else {
            let res = disj.borrow().to_conjunctive_normal_form();
            res
          }

        }

        else if let (Token::Conjunction, Token::Value(_)) = (&left.value, &right.value)
        {
          // (a and b) or c => (a or c) and (b or c)
          let a = left.left.as_ref().unwrap();
          let b = left.right.as_ref().unwrap();
          let c = right_ref;

          let disj_1 = Node::new(Token::Disjunction, Some(a.clone()), Some(c.clone()));
          let disj_2 = Node::new(Token::Disjunction, Some(b.clone()), Some(c.clone()));

          let disj_1 = disj_1.borrow().to_conjunctive_normal_form();
          let disj_2 = disj_2.borrow().to_conjunctive_normal_form();

          Node::new(Token::Conjunction, Some(disj_1), Some(disj_2))
        }

        else if let (Token::Value(_), Token::Value(_)) = (&left.value, &right.value)
        {
          let a = left.to_conjunctive_normal_form();
          let b = right.to_conjunctive_normal_form();

          Node::new(Token::Disjunction, Some(a), Some(b))
        }

        else if let (Token::Disjunction, Token::Conjunction) |
                    (Token::Value(_), Token::Disjunction) |
                    (Token::Value(_), Token::Conjunction) = (&left.value, &right.value)
        {
          let a = right_ref.clone();
          let b = left_ref.clone();
          let new_node = Node::new(Token::Disjunction, Some(a), Some(b));
          
          let res = new_node.borrow().to_conjunctive_normal_form();
          res
        }

        else
        {
          let a = left.to_conjunctive_normal_form();
          let b = right.to_conjunctive_normal_form();

          Node::new(Token::Disjunction, Some(a), Some(b))
        }

      },
      Token::Conjunction => {
        let left = self.left.as_ref().unwrap().borrow().to_conjunctive_normal_form();
        let right = self.right.as_ref().unwrap().borrow().to_conjunctive_normal_form();

        Node::new(Token::Conjunction, Some(left), Some(right))
      },
      Token::Negation => {
        let left = self.left.as_ref().unwrap().borrow().to_conjunctive_normal_form();

        Node::new(Token::Negation, Some(left), None)
      }
      Token::Value(_) => {
        Node::new(self.value.clone(), None, None)
      },
      _ => {
        panic!("Unexpected expression probably the expression isn't in negation normal form.");
      }
    }

  }


  pub fn as_string(&self, counter : usize) -> String {
    let colors = vec!["\u{001b}[31m", "\u{001b}[32m", "\u{001b}[33m", "\u{001b}[34m", "\u{001b}[35m"];
    let r_color = "\u{001b}[0m";
    let color = colors[counter % colors.len()];

    match self.value {
      Token::Conjunction |
      Token::Disjunction |
      Token::LogicalEquivalence |
      Token::MaterialCondition |
      Token::ExclusiveDisjunction => {
        let a = self.left.as_ref().unwrap().borrow().as_string(counter + 1);
        let b = self.right.as_ref().unwrap().borrow().as_string(counter + 1);
        format!("{}({}{} {} {}{}){}", color, r_color, a, self.value.get_value(), b, color, r_color)
      },
      Token::Negation => {
        let a = self.left.as_ref().unwrap().borrow().as_string(counter + 1);
        format!("!{}", a)
      },
      Token::Value(_) => {
        self.value.get_value().to_string()
      }
    }
  }

  pub fn to_polish_form(&self) -> String {
    match self.value {
      Token::Conjunction |
      Token::Disjunction |
      Token::ExclusiveDisjunction |
      Token::LogicalEquivalence |
      Token::MaterialCondition => {
        let a = self.left.as_ref().unwrap().borrow().to_polish_form();
        let b  = self.right.as_ref().unwrap().borrow().to_polish_form();
        format!("{}{}{}", a, b, self.value.get_value())
      },
      Token::Negation => {
        let a = self.left.as_ref().unwrap().borrow().to_polish_form();
        format!("{}!", a)
      }
      Token::Value(c) => {
        format!("{}", c)
      }
    }
  }

}

pub struct Tree {
  root : Option<Rc<RefCell<Node>>>
}


impl Tree {
  
  pub fn new() -> Tree
  {
    Tree {
      root : None
    }
  }

  pub fn new_iter<I>(iter : I) -> Tree 
  where I : Iterator<Item = char>
  {
    // if iter.peekable().peek().is_none() {
    //   return Tree {root : None};
    // }

    let mut stack : Stack<Rc<RefCell<Node>>>  = Stack::new();
  
    for c in iter {
  
      let token = Token::new(c);
      
      let mut pop_stack = || {
        stack.pop().expect("Expected a value but found the stack empty!")
      };
  
      match token {
        Token::Negation => {
          let v1  = pop_stack();
          let left = Some(v1);
          stack.push(Node::new(token, left, None));
        },
  
        Token::Conjunction |
        Token::Disjunction |
        Token::ExclusiveDisjunction |
        Token::MaterialCondition |
        Token::LogicalEquivalence => {
          let v1 = pop_stack();
          let v2 = pop_stack();
          let left = Some(v2);
          let right= Some(v1);
          stack.push(Node::new(token, left, right));
        }, 
  
        Token::Value(_) => {
          stack.push(Node::new(token, None, None))
        }
      }
  
    }
    
    if stack.size() == 1 {
      let root = stack.pop().unwrap();
      return Tree {
        root : Some(root)
      };
    }
  
    panic!("Invalid expression")
  }

  pub fn get_height(&self) -> usize {
    if let Some(node) = &self.root {
      return node.borrow().get_height();
    }
    return 0;
  }

  pub fn print(&self) {
    let mut stack : VecDeque<Option<Rc<RefCell<Node>>>> = VecDeque::new();
    let item_buf_size = 1;
    let height = self.get_height();
    let mut buf_size = item_buf_size * (1 << height);
    let mut level = 0;
    let mut level_items = 1;

    let print_center = |size : usize, a : &str| {
      let mut b = a.to_owned();
      if a.len() > size {
        b = a[0..size].to_owned();
      }
      let half = (size + b.len()) / 2;
      print!("{}", " ".repeat(half));
      print!("{}", b);
      print!("{}", " ".repeat(size - half - b.len()));
    };

    stack.push_back(self.root.clone());

    while level < height {
      let val = stack.pop_front().unwrap();

      if let Some(val) = &val {
        let value = val.borrow().value.get_value().to_string();
        stack.push_back(val.borrow().left.clone());
        stack.push_back(val.borrow().right.clone());
        print_center(buf_size, &value);
      } else {
        stack.push_back(None);
        stack.push_back(None);
        print_center(buf_size, ".");
      }
      
      level_items -= 1;
      if level_items == 0 {
        println!("");
        buf_size /= 2;
        level += 1;
        level_items = 1 << level;
      }
    }
    

  }


  pub fn to_basic_logic_operators(&mut self) {
    if let Some(root) = self.root.as_ref() {
      let res = root.borrow().to_basic_logic_operators();
      self.root = Some(res);
    }
  }

  pub fn to_negation_normal_form(&mut self) {
    if let Some(root) = self.root.as_ref() {
      let res = root.borrow().to_basic_logic_operators();
      let res = res.borrow().to_negation_normal_form();
      self.root = Some(res);
    }
  }

  pub fn to_conjunctive_normal_form(&mut self) {
    self.to_negation_normal_form();
    self.print();
    self.print_as_string();
    if let Some(root) = self.root.as_ref() {
      let res = root.borrow().to_conjunctive_normal_form();
      self.root = Some(res);
    }
  }


  pub fn print_as_string(&self) {
    if let Some(root) = self.root.as_ref() {
      let res = root.borrow().as_string(0);
      println!("{}", res);
    } else {
      println!("None")
    }
  }

  pub fn to_polish_form(&self) -> String {
    if let Some(root) = self.root.as_ref() {
      root.borrow().to_polish_form()
    } else {
      String::new()
    }
  }
  

}

