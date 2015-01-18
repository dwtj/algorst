extern crate algorst;

use algorst::binary_tree::Tree;


fn double(x: i32) -> i32 {
    2 * x
}


#[test]
fn new_works() {
    let empty: Tree<u32> = Tree::new();
}

#[test]
fn peek_works() {
    let elem = 42;

    let mut t: Tree<u32> = Tree::new();
    assert_eq!(t.is_nil(), true);

    t.give(elem);
    assert_eq!(t.is_nil(), false);

    assert_eq!(t.peek(), elem);
    assert_eq!(t.is_nil(), false)
}

/*
fn make_left_works() {
    let mut t: Tree<u32> = Tree::new();
    t.make_left(5);
}
*/

#[test]
fn apply_to_nil_works() {
    let mut empty: Tree<i32> = Tree::new();
    //let double = |&x: i32| (*x) * 2;

    empty.apply(double);
    assert_eq!(empty.is_nil(), true);
}

#[test]
fn apply_to_root_works() {
    //let double = |&x: i32| (*x) * 2;
    let elem = 42;
    let mut t: Tree<i32> = Tree::new();

    t.give(elem);
    t.apply(double);

    assert_eq!(t.peek(), double(elem))
}

/*
#[test]
fn apply_to_tree_works() {
    let mut t: Tree<i32> = Tree::new();
    let 
}
*/
