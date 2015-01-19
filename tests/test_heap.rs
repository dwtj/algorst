extern crate algorst;

use algorst::heap::Heap;

#[test]
fn test_single_push_and_pop() {
    let mut heap: Heap<u32> = Heap::new();
    heap.push(42);
    assert_eq!(heap.pop().unwrap(), 42);
}
