extern crate algorst;

use algorst::heap::Heap;

#[test]
fn test_single_push() {
    let mut heap: Heap<u32> = Heap::new();
    heap.push(42);
    assert_eq!(heap.len(), 1);
}

#[test]
fn test_single_push_and_pop() {
    let mut heap: Heap<u32> = Heap::new();
    heap.push(42);
    let popped = heap.pop();
    assert_eq!(popped.unwrap(), 42);
}
