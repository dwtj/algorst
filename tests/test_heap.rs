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

fn log_heap(heap: &Heap<u32>, p: &Path) {
    let mut dot_file = match File::create(p) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    graphviz::render(heap, &mut dot_file);
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
    let mut actual = Vec::new();
    for idx in (0..expected.len()) {
        actual.push(heap.pop().unwrap());
    }

    let p = Path::new("tests/logs/heap_push_and_pop.log");
    let mut f = File::create(&p).unwrap();
    f.write_fmt(format_args!("expected: {:?}\n", expected));
    f.write_fmt(format_args!("actual:   {:?}\n", actual));
    assert_eq!(expected, actual);
    assert_eq!(heap.len(), 0);
}

#[test]
fn test_graphviz_render_empty() {
    let heap = Heap::new();
    let p = Path::new("tests/logs/empty_heap.dot");
    log_heap(&heap, &p);
}

#[test]
fn test_graphviz_render() {
    let heap = make_heap();
    let p = Path::new("tests/logs/heap.dot");
    log_heap(&heap, &p);
}
