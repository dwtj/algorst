extern crate algorst;

use algorst::lists::List;

#[test]
fn new_works() {
    let xs: List<u32> = List::new();
}

#[test]
fn is_empty_works() {
    let xs: List<u32> = List::new();
    assert!(xs.is_empty())
}

#[test]
fn prepend_works() {
    let mut xs: List<u32> = List::new();
    xs = xs.prepend(4);
    xs = xs.prepend(3);
    assert_eq!(xs.head(), 3);
}

#[test]
fn tail_works() {
    let mut xs: List<u32> = List::new();
    let ys = vec!(0, 1, 2, 3, 4);

    // Prepend each the `ys` to `xs`, one-by-one.
    for y in ys.iter().rev() {
        xs = xs.prepend(*y);
    }

    // Repeatedly pop each `x` and check to see that it matches the expected `y`.
    for y in ys.iter() {
        let x = xs.head();
        assert_eq!(*y, xs.head());
        xs = xs.tail();
    }

    assert!(xs.is_empty());
}
