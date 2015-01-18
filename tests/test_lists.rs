extern crate algorst;

use algorst::lists::List;

#[test]
fn new_works() {
    let xs: List<u32> = List::new();
}

#[test]
fn prepend_works() {
    let mut xs: List<u32> = List::new();
    xs = xs.prepend(4);
    xs = xs.prepend(3);
    assert_eq!(xs.head(), 3);
}

#[test]
fn is_empty_works() {
    let xs: List<u32> = List::new();
}
