pub struct Counter {
    pub value: i32,
}

impl Counter {
    pub fn new(value: i32) -> Counter {
        Counter { value }
    }
    pub fn inc(&mut self) -> i32 {
        self.value += 1;
        self.value
    }
    pub fn val(self) -> i32 {
        self.value
    }
    pub fn reset(&mut self) -> i32 {
        self.value = 0;
        self.value
    }
}
