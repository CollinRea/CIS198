#[derive(Debug)]
pub struct BST {
  root: Link,
}

impl BST {
  pub fn new() -> BST {
    BST { root: Link::Empty }
  }
}

#[derive(Debug)]
pub enum Link {
  Empty,
  More(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
  val: i32,
  left: Link,
  right: Link,
}