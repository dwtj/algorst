pub mod boxes {
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
}
