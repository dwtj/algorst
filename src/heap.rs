/// Implements a heap data structure (a.k.a. a priority queue).
///
/// In this implementation, heap is "top-heavy" meaning that the root node is
/// the node with the highest value in the heap. The relative priority of
/// two nodes is determined via the `cmp` function defined over type of the
/// heap's elements (i.e. the generic type `T`).
///
/// The allocated memory used to store the elements of the heap grows (and
/// shrinks) as necessary.
///
/// Method naming conventions generally follow those found in `std::vec::Vec`.
///
/// The heap can contain more than one element with the same priority. No
/// guarantees are made about the order in which elements with equivalent
/// priorities are popped from the queue.

// This data structure is implemented as a vector-backed binary heap. (The
// parent-child relationships are therefore not stored via pointers between
// nodes, but using logical connections between NodeIdx values.
// 
// See [Wikipedia](http://en.wikipedia.org/wiki/Heap_%28data_structure%29)
// for overview of this implementation strategy.
//
// A simple C implementation of a binary heap is available from
// [here](https://github.com/dale48/levawc). This code is adapted from Chapter
// 10 of O'Reilly book *Mastering Algorithms with C*. This chapter also served
// as a guide while implementing this module.

use std::ptr;
use std::fmt;
use std::io;

type NodeIdx = usize;

pub struct Heap<T: Ord> {
    store: Vec<T>,
}

enum ChildType {
    Left,
    Right
}

impl fmt::String for ChildType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChildType::Left  => write!(f, "Left"),
            ChildType::Right => write!(f, "Right")
        }
    }
}

fn left_child(i: NodeIdx) -> NodeIdx {
    2 * i + 1
}

fn right_child(i: NodeIdx) -> NodeIdx {
    2 * i + 2
}

impl<T: Ord> Heap<T> {
    /// Creates a new empty heap.
    pub fn new() -> Heap<T> {
        Heap { store: Vec::new() }
    }

    /// Creates a new empty heap which has initially allocated enough memory
    /// for the given number of elements.
    pub fn with_capacity(capacity: usize) -> Heap<T> {
        Heap { store: Vec::with_capacity(capacity) }
    }

    /// Adds the given element to the heap.
    pub fn push(&mut self, elem: T) {
        let len = (*self).store.len();
        (*self).store.push(elem);
        let insert_idx: NodeIdx = len as NodeIdx;
        (*self).percolate_up(insert_idx);
    }

    /// Removes from the heap an element with the largest priority of all in
    /// the heap. This element is then returned wrapped returns it wrapped in
    /// an `Option<T>`. If there are no elements in the heap, then `None` is
    /// returned.
    pub fn pop(&mut self) -> Option<T> {
        match (*self).store.len() {
            0 => None,
            1 => (*self).store.pop(),
            _ => {
                let rv = (*self).store.swap_remove(0);
                (*self).percolate_down(0);
                Some(rv)
            }
        }
    }

    /// Returns the number of elemens in the heap.
    pub fn len(&self) -> usize {
        (*self).store.len()
    }


    /// Takes the index of a node and returns the index of its parent. Returns
    /// `None` if the given node has no such parent (i.e. the given node is
    /// the root.
    ///
    /// The function panics if the given index is not valid.
    fn parent(&self, idx: NodeIdx) -> Option<NodeIdx> {
        if (*self).is_valid(idx) {
            if idx == 0 { None } else { Some((idx - 1) / 2) }
        } else {
            panic!("Heap.parent({}): given `idx` not in the heap.", idx)
        }
    }

    fn left_child(&self, parent: NodeIdx) -> Option<NodeIdx> {
        (*self).child(ChildType::Left, parent)
    }

    fn right_child(&self, parent: NodeIdx) -> Option<NodeIdx> {
        (*self).child(ChildType::Right, parent)
    }

    /// Takes the index of a node and returns the index of indicated child.
    /// Returns `None` if the given node has no such child.
    ///
    /// The function panics if the given index is not valid.
    fn child(&self, ct: ChildType, parent: NodeIdx) -> Option<NodeIdx> {
        if (*self).is_valid(parent) {
            let child: NodeIdx = match ct {
                ChildType::Left  => left_child(parent),
                ChildType::Right => right_child(parent)
            };
            if child < (*self).store.len() {
                Some(child) }
            else {
                None
            }
        } else {
            panic!("Heap.child({}, {}): the given `idx` is not in the Heap.",
                   ct, parent)
        }
    }

    /// Starting from the given `NodeIdx`, recursively move an element up the
    /// heap until the heap property has been restored all along this node's
    /// ancestor path.
    fn percolate_up(&mut self, child: NodeIdx) {
        let maybe_parent = (*self).parent(child);
        match maybe_parent {
            None => {
                // Do nothing: The given `child` has no parent because it is
                // the root node.
                return
            },
            Some(parent) => {
                if (*self).is_violating(parent, child) {
                    (*self).swap(parent, child);
                    (*self).percolate_up(parent)
                } else {
                    // Do nothing: the two nodes are already ordered correctly.
                    return
                }
            }
        }
    }

    /// Starting from the given `NodeIdx`, recursively move an element down
    /// the heap until the heap property has been restored in the entire
    /// sub-heap.
    ///
    /// (For the heap property to be restored to the entire sub-heap being
    /// re-heapified, the only element which may be violating the heap-property
    /// is the node indicated by the given `NodeIdx`.)
    fn percolate_down(&mut self, parent: NodeIdx) {
        match ((*self).left_child(parent), (*self).right_child(parent)) {
            (None, None)        => return,
            (None, Some(right)) => panic!("Heap can't only have right child."),
            (Some(left), None)  => {
                if (*self).is_violating(parent, left) {
                    (*self).swap_down(parent, left)
                }
            },
            (Some(left), Some(right)) => {
                match ((*self).is_violating(parent, left),
                       (*self).is_violating(parent, right)) {
                    (false, false) => return,
                    (false, true)  => (*self).swap_down(parent, right),
                    (true,  false) => (*self).swap_down(parent, left),
                    (true,  true)  => {
                        // Since both of the parent's children are violating
                        // the heap property, choose which child should be
                        // swapped with the parent, such that the heap property
                        // will not be violated after the swap. (That is, the
                        // greater of the two will need to become the parent of
                        // the other.)
                        if (*self).store[left] >= (*self).store[right] {
                            (*self).swap_down(parent, left)
                        } else {
                            (*self).swap_down(parent, right)
                        }
                    }
                }
            }
        }
    }

    /// Helper function for `percolate_down()`.
    fn swap_down(&mut self, parent: NodeIdx, child: NodeIdx) {
        (*self).swap(parent, child);
        (*self).percolate_down(child);
    }

    /// Checks to see whether the given parent-child nodes are violating the
    /// heap property.
    ///
    /// Panics if either index is out of bounds. Panics if the given parent is
    /// not actually the parent of the given child.
    fn is_violating(&self, parent: NodeIdx, child: NodeIdx) -> bool{
        if parent == (*self).parent(child).unwrap() {
            (*self).store[parent] < (*self).store[child]
        } else {
            panic!("Given parent is not actually the parent of this child.")
        }
    }

    fn is_valid(&self, idx: NodeIdx) -> bool {
        idx < (*self).store.len()
    }

    /// Swaps the data stored at the two inciated heap nodes.
    ///
    /// Does nothing if the two indices are the same. Panics if either index is
    /// invalid.
    fn swap(&mut self, a: NodeIdx, b: NodeIdx) {
        if a != b {
            unsafe {
                let pa: *mut T = &mut self.store[a];
                let pb: *mut T = &mut self.store[b];
                ptr::swap(pa, pb);
            }
        }
    }
}

/*
impl<T: Ord> Clone for Heap<T> {
    pub fn clone(&self) -> Heap<T> {
        Heap { store: (*self).store.clone() }
    }
}
*/
