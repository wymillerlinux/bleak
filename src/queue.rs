struct Queue<T> {
    pub queue: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            queue: Vec::new()
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    pub fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    pub fn length(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

    pub fn peek_at(&self, item: T) -> Option<&T> {
        self.queue.get(item)
    }
}
