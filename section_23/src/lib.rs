use std::collections::LinkedList;

pub struct Queue<T> {
    element: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            element: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.element.push_back(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.element.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.element.front()
    }

    pub fn length(&self) -> usize {
        self.element.len()
    }

    pub fn is_empty(&self) -> bool {
        self.element.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn basics() {
        let mut queue: Queue<i8> = Queue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);
        assert_eq!(queue.length() , 3);
        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.length() , 2);
        queue.enqueue(10);
        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.peek(), Some(&30));
        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.is_empty(), false);
        queue.dequeue();
        assert_eq!(queue.is_empty(), true);
    }
}