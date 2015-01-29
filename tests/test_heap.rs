extern crate algorst;
extern crate graphviz;

use algorst::heap::Heap;
use std::old_io::Writer;
use std::old_io::{File, Open, ReadWrite};

fn make_heap() -> Heap<u32> {
    let mut heap: Heap<u32> = Heap::new();
    let to_add = vec!(42, 342, 352, 541, 511, 0, 531, 342);
    for idx in (0..to_add.len()) {
        heap.push(to_add[idx]);
    }
    heap
}

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

#[test]
fn test_more_push_and_pop() {
    let mut heap: Heap<u32> = Heap::new();

    heap.push(42);
    heap.pop();

    let to_add = vec!(42, 342, 352, 541, 511, 0, 531, 342);
    for idx in (0..to_add.len()) {
        heap.push(to_add[idx]);
    }

    let expected = vec!(541, 531, 511, 352, 342, 342, 42, 0);
    for idx in (0..expected.len()) {
        let popped = heap.pop();
        assert_eq!(heap.pop().unwrap(), expected[idx]);
    }

    assert!(heap.empty());
}

#[test]
fn test_graphviz_render() {
    let heap = make_heap();

    let p = Path::new("tests/logs/heap.dot");
    let mut dot_file = match File::create(&p) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    graphviz::render(&heap, &mut dot_file);
}
