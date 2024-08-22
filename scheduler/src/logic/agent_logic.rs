use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct PriorityQueue<T> {
    heap: BinaryHeap<Reverse<T>>,  // Using `Reverse` to convert max-heap to min-heap
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    // Insert an element into the queue
    pub fn push(&mut self, item: T) {
        self.heap.push(Reverse(item));
    }

    // Remove and return the element with the highest priority (smallest value)
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(item)| item)
    }

    // Peek at the element with the highest priority without removing it
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|Reverse(item)| item)
    }

    // Return the number of elements in the queue
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    // Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    // Sort and return all elements in sorted order (destructive operation)
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut sorted_vec = Vec::with_capacity(self.len());  // Pre-allocated capacity to improve performance.
        while let Some(item) = self.pop() {
            sorted_vec.push(item);
        }
        sorted_vec
    }
}

// Example usage
fn main() {
    let mut pq = PriorityQueue::new();
    pq.push(5);
    pq.push(2);
    pq.push(8);
    pq.push(3);

    println!("Sorted elements: {:?}", pq.into_sorted_vec());
}
