use std::{mem, fmt};
use std::fmt::{Display, Debug};

#[derive(PartialEq, Eq, Clone)]
pub enum BinaryTree<T> {
  Leaf,
  Node(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>)
}



impl<T: Debug + Display + PartialOrd> BinaryTree<T> {
  pub fn len(&self) -> usize {
    match self {
      BinaryTree::Leaf => {0}
      BinaryTree::Node(_t, l, r) => {
        1 + l.len() + r.len()
      }
    }
  }

  pub fn to_vec(&self) -> Vec<&T> {
    match self {
      BinaryTree::Leaf => {Vec::new()}
      BinaryTree::Node(t, l, r) => {
        let left_vector = l.to_vec();
        let right_vector = r.to_vec();
        let mut result = Vec::new();
        left_vector.iter().for_each(|elem| {
          result.push(*elem)
        });
        result.push(t);
        right_vector.iter().for_each(|elem| {
          result.push(*elem)
        });
        result
      }
    }
  }


  pub fn sorted(&self) -> bool {
    let vector = self.to_vec();
    match vector.first() {
      Option::None => {true}
      Option::Some(head) => {
        let mut iter_value = head;
        let mut should_continue = true;
        vector .iter().for_each(|value | {
          if should_continue {
            if iter_value > value {
               should_continue = false;
            }  else {
              let iter_value = value;
            }
          }

        });
        return true
      }

    }

  }






  pub fn insert(&mut self, t: T) {
    match self {
      BinaryTree::Leaf =>  (*self =  BinaryTree::Node(t, Box::new(BinaryTree::Leaf), Box::new(BinaryTree::Leaf))),
      BinaryTree::Node(current_t, left, right) =>
        if *current_t < t {
          right.insert(t)
        } else {
          left.insert(t)
        }


    }
  }

//  This function returns the smallest element greater than or equal to the query element. If no such element exists, then return None.
  fn search_helper<'a>(&self, query: &T, sub_result: &'a Option<&T>) -> &'a Option<&T> {
    match self {
        BinaryTree::Leaf => {sub_result}
        BinaryTree::Node(other, left, right) => {
          if other >= query {
            left.search_helper(query, &Some(other))
          } else {
            right.search_helper(query, sub_result)
          }
        }
    }
  }

  pub fn search(&self, query: &T) -> Option<&T> {
    self.search_helper(query, None)
  }

  pub fn rebalance(&mut self) {
    unimplemented!()
  }


  // Adapted from https://github.com/bpressure/ascii_tree
  fn fmt_levels(&self, f: &mut fmt::Formatter<'_>, level: Vec<usize>) -> fmt::Result {
    use BinaryTree::*;
    const EMPTY: &str = "   ";
    const EDGE: &str = " └─";
    const PIPE: &str = " │ ";
    const BRANCH: &str = " ├─";

    let maxpos = level.len();
    let mut second_line = String::new();
    for (pos, l) in level.iter().enumerate() {
      let last_row = pos == maxpos - 1;
      if *l == 1 {
        if !last_row { write!(f, "{}", EMPTY)? } else { write!(f, "{}", EDGE)? }
        second_line.push_str(EMPTY);
      } else {
        if !last_row { write!(f, "{}", PIPE)? } else { write!(f, "{}", BRANCH)? }
        second_line.push_str(PIPE);
      }
    }

    match self {
      Node(s, l, r) => {
        let mut d = 2;
        write!(f, " {}\n", s)?;
        for t in &[l, r] {
          let mut lnext = level.clone();
          lnext.push(d);
          d -= 1;
          t.fmt_levels(f, lnext)?;
        }
      }
      Leaf => {write!(f, "\n")?}
    }
    Ok(())
  }
}

impl<T: Debug + Display + PartialOrd> fmt::Debug for BinaryTree<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.fmt_levels(f, vec![])
  }
}

#[cfg(test)]
mod test {
  use lazy_static::lazy_static;
  use super::BinaryTree::*;
  use crate::BinaryTree;

  lazy_static! {
    static ref TEST_TREE: BinaryTree<&'static str> = {
      Node(
        "B",
        Box::new(Node("A", Box::new(Leaf), Box::new(Leaf))),
        Box::new(Node("C", Box::new(Leaf), Box::new(Leaf))))
    };
  }



  #[test]
  fn len_test() {
    assert_eq!(TEST_TREE.len(), 3);
  }

  #[test]
  fn to_vec_test() {
    assert_eq!(TEST_TREE.to_vec(), vec![&"A", &"B", &"C"]);
  }

  #[test]
  fn sorted_test() {
    let mut t = TEST_TREE.clone();
    assert!(t.sorted());

    t = Node("D", Box::new(Leaf), Box::new(t));
    assert!(!t.sorted());
  }

  #[test]
  fn insertion_test() {
    let mut t = TEST_TREE.clone();
    t.insert("E");
    assert!(t.sorted());
  }

  #[test]
  fn search_test() {
    let mut t= TEST_TREE.clone();
    t.insert("E");
    assert!(t.search(&"D") == Some(&"E"));
    assert!(t.search(&"C") == Some(&"C"));
    assert!(t.search(&"F") == None);
  }

  #[test]
  fn rebalance1_test() {
    let mut t = Node(
      "D",
      Box::new(Node(
        "B",
        Box::new(Node(
          "A", Box::new(Leaf), Box::new(Leaf))),
        Box::new(Node(
          "C", Box::new(Leaf), Box::new(Leaf))))),
      Box::new(Node(
        "E", Box::new(Leaf), Box::new(Leaf))));

    let t2 = Node(
      "C",
      Box::new(Node(
        "B",
        Box::new(Node(
          "A", Box::new(Leaf), Box::new(Leaf))),
        Box::new(Leaf))),
      Box::new(Node(
        "D",
        Box::new(Leaf),
        Box::new(Node(
          "E", Box::new(Leaf), Box::new(Leaf)))
      )));

    t.rebalance();
    assert_eq!(t, t2);
  }

  #[test]
  fn rebalance2_test() {
    let mut t = Node(
      "A",
      Box::new(Leaf),
      Box::new(Node(
        "B",
        Box::new(Leaf),
        Box::new(Node(
          "C",
          Box::new(Leaf),
          Box::new(Node(
            "D",
            Box::new(Leaf),
            Box::new(Leaf))))))));

    let t2 = Node(
      "B",
      Box::new(Node("A", Box::new(Leaf), Box::new(Leaf))),
        Box::new(Node(
          "C",
          Box::new(Leaf),
          Box::new(Node(
            "D",
            Box::new(Leaf),
            Box::new(Leaf))))));

    t.rebalance();
    assert_eq!(t, t2);
  }

  #[test]
  fn rebalance3_test() {
    let mut t = Node(
      "E",
      Box::new(Node(
        "B",
        Box::new(Leaf),
        Box::new(Node(
          "D",
          Box::new(Node(
            "C", Box::new(Leaf), Box::new(Leaf))),
          Box::new(Leaf))))),
      Box::new(Node(
        "F", Box::new(Leaf), Box::new(Leaf))));

    let t2 = Node(
      "D",
      Box::new(Node(
        "B",
        Box::new(Leaf),
        Box::new(Node("C", Box::new(Leaf), Box::new(Leaf))))),
      Box::new(Node(
        "E",
        Box::new(Leaf),
        Box::new(Node("F", Box::new(Leaf), Box::new(Leaf))))));

    t.rebalance();
    assert_eq!(t, t2);
  }
}
