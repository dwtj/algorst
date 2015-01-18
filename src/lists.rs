use std::rc::Rc;

pub struct Node<T: Clone> {
    elem: T,
    link: Rc<List<T>>,
}

//pub type List<T> = Option<Node<T>>;
pub enum List<T: Clone> {
    Empty,
    Head(Node<T>)
}

impl <T: Clone> List<T> {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            List::Empty => true,
            _           => false
        }
    }

    pub fn head(&self) -> T {
        match *self {
            List::Empty   => panic!(),
            List::Head(ref n) => n.elem.clone()
        }
    }

    pub fn prepend(self, elem: T) -> Self {
        let n = Node { elem: elem, link: Rc::new(self) };
        List::Head(n)
    }

    /*
    pub fn tail(&self) -> List<T> {
        match *self {
            List::Empty   => panic!(),
            List::Head(ref n) => n.link.clone()
        }
    }
    */
}
