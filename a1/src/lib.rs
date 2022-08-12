#[derive(Debug)]
pub struct A<T> {
    value: T,
}

impl<T: Copy> A<T> {
    pub fn new(value: T) -> Self {
        A { value }
    }

    pub fn value(&self) -> T {
        self.value
    }
}
