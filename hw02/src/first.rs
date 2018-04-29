#[derive(Debug, PartialEq)]
pub struct BST {
  root: Link,
}

impl BST {
  pub fn new() -> BST {
    BST { root: Link::Empty }
  }
}

#[derive(Debug, PartialEq)]
enum Link {
  Empty,
  More(Box<Node>),
}

#[derive(Debug, PartialEq)]
struct Node {
  val: i32,
  left: Link,
  right: Link,
}


-----  TESTS -----

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn run_new() {
    let test_bst = BST { root: Link::Empty };
    assert_eq!(test_bst, BST::new());
  }
}