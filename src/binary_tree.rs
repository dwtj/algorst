pub struct Node<T: Clone> {
    elem: T,
    left: Box<Tree<T>>,
    right: Box<Tree<T>>,
}

/*
impl<T: Clone> Drop for Node<T> {
    fn drop(self) {
        println!("> Dropping {}", self.name);
    }
}
*/

impl<T: Clone> Node<T> {
    /// Allocates a new binary tree node to contain the given element.
    pub fn new(elem: T) -> Node<T> {
        Node {
            elem: elem,
            left: box Tree::Nil,
            right: box Tree::Nil
        }
    }
}

pub enum Tree<T: Clone> {
    Nil,
    Node(Node<T>)
}


impl<T: Clone> Tree<T> {
    /// Creates a new tree which is empty.
    pub fn new() -> Tree<T> {
        Tree::Nil
    }

    pub fn is_nil(&self) -> bool {
        match *self {
            Tree::Nil => true,
            _         => false
        }
    }

    /// Returns a clone of the element stored in the root node.
    ///
    /// Panics if there is no root node.
    pub fn peek(&self) -> T {
        match *self {
            Tree::Nil => panic!(),
            Tree::Node(ref n) => (*n).elem.clone()
        }
    }

    /*
    /// The given element is cloned, and this clone is stored at the root of
    /// the tree.
    ///
    /// If the tree is `Nil`, then a node is made in-place to hold the element.
    pub fn set(&mut self, &elem: T) {
        match *self {
            Tree::Nil => Node::new(elem),
            Tree::Node(ref mut n) => *n.elem = *elem.clone()
        };
    }
    */

    /// The given `elem` is stored as the `elem` of the tree's root node. The
    /// tree takes ownership of the given node.
    ///
    /// If the tree is `Nil`, then a node is made in-place to hold the `elem`.
    pub fn give(&mut self, elem: T) {
        match *self {
            Tree::Nil => *self = Tree::Node(Node::new(elem)),
            Tree::Node(ref mut n) => (*n).elem = elem
        };
    }



    /*
    /// Saves the given element in the data field of a leaf node. This node is
    /// then saved as the tree's left subtree. This calls `panic!()` if `self`
    /// is `Nil`.
    pub fn make_left(& mut self, elem: T) {

    }
    */

    /// Applies the given function to each element of the tree and saves the
    /// results in-place.
    pub fn apply<F: Fn(T) -> T>(&mut self, f: F) {
        match *self {
            Tree::Nil => (),
            Tree::Node(ref mut node) => (*node).elem = f((*node).elem.clone())
        }
    }
}
