#[derive(Debug)]
pub struct A {
    value: usize,
}

impl A {
    pub fn new(value: usize) -> Self {
        A { value }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}
