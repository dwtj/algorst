extern crate algorst;

use algorst::MyBox;

#[test]
fn i32_works() {
    let b: MyBox<i32> = MyBox::new(4i32);
    assert_eq!(4, b.get());
}

#[test]
fn str_works() {
    let b: MyBox<&str> = MyBox::new("foo");
    assert_eq!("foo", b.get());
}
