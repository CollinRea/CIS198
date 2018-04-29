/* 
  Binary Search Tree implementation

  Usage:
    let bst = BST::new()
      -- create a new empty Binary Search Tree

    bst.insert(val) 
      -- insert some value into the Binary Search Tree
      -- returns true if successful or false if it fails
    
    bst.search(val)
      -- search if a value is in the Binary Search Tree
      -- returns true if it finds it, or false if not

*/

// ----- DATA STRUCTURE -----

#[derive(Debug, PartialEq, Clone)]
pub struct BST {
  root: Link,
}

impl BST {
  pub fn new() -> BST {
    BST { root: Link::Empty }
  }
  pub fn insert(&mut self, v: i32) -> bool {
    self.root.insert(v)
  }
  pub fn search(&self, v: i32) -> bool {
    self.root.search(v)
  }
}

#[derive(Debug, PartialEq, Clone)]
enum Link {
  Empty,
  More(Box<Node>),
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
  val: i32,
  left: Link,
  right: Link,
}


// ----- METHODS -----

impl Link {
  fn insert(&mut self, v: i32) -> bool {
    let prev_link = self.clone();
    match self {
      Link::Empty => {
        let new_node = Node {
          val: v,
          left: Link::Empty,
          right: Link::Empty,
        };
        *self = Link::More(Box::new(new_node))
      },
      Link::More(box_node) => {
        if box_node.val > v {
          box_node.left.insert(v);
        }
        if box_node.val < v {
          box_node.right.insert(v);
        }
      },
    };
    prev_link != *self
  }
}

// link.search(i32) -> bool (~15 lines)
//     If searching an empty link, return false; the element can't be found.
//     If searching a non-empty link:
//         return true if the element is in this node; otherwise,
//         recurse to the left if the target value is less than the node's value
//         recurse to the right if the target value is greater than the node's value

impl Link {
  fn search(&self, v: i32) -> bool {
    false
  }
}


// ----- TESTS -----

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn run_new() {
    let test_bst = BST { root: Link::Empty };
    let new_bst = BST::new();
    assert_eq!(test_bst, new_bst);
    
  }

  #[test]
  fn test_insert() {
    let empty_bst = BST::new();
    let mut test_bst = BST::new();
    assert!(test_bst.insert(1));
    assert_ne!(empty_bst, test_bst);
    
    // Make sure if you try to enter same value, it returns false
    // And BST is unchanged
    test_bst.insert(2);
    let prev_bst = test_bst.clone();
    assert!(!test_bst.insert(2));
    assert_eq!(prev_bst, test_bst);
  }

  #[test]
  fn test_search() {
    let mut bst = BST::new();
    bst.insert(1);
    assert!(bst.search(1));
  }
}