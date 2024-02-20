use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::VecDeque;

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
  parent : Weak<RefCell<Node>>,
  left : Option<Rc<RefCell<Node>>>,
  right : Option<Rc<RefCell<Node>>>
}



impl Node {
  pub fn new(t : Token, left : Option<Rc<RefCell<Node>>>, right : Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {

  
    let new_node = Node {
      value : t,
      parent : Weak::new(),
      left : left,
      right : right
    };

    let new_node = Rc::new(RefCell::new(new_node));

    if let Some(l) = &new_node.borrow().left {
      l.borrow_mut().parent = Rc::downgrade(&new_node);
    }
    if let Some(r) = &new_node.borrow().right {
      r.borrow_mut().parent = Rc::downgrade(&new_node);
    }

    return new_node;
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

}

pub struct Tree {
  root : Option<Rc<RefCell<Node>>>
}


impl Tree {
  
  pub fn new<I>(iter : I) -> Tree 
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
          let left = Some(v1);
          let right= Some(v2);
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
    let item_buf_size = 2;
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
}

