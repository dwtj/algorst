pub struct MyBox<T: Clone> {
    data: T,
}

impl<T: Clone> MyBox<T> {
    pub fn new(d: T) -> Self {
        MyBox { data: d }
    }

    pub fn get(&self) -> T {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
