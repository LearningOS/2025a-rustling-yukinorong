/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // Add the new element at the end
        self.count += 1;
        if self.count >= self.items.len() {
            self.items.push(value);
        } else {
            self.items[self.count] = value;
        }
        
        // Bubble up to maintain heap property
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // If the comparator says parent should come before child, we're done
            // For min-heap: comparator(a,b) = a < b, so if parent < child, we're good
            // For max-heap: comparator(a,b) = a > b, so if parent > child, we're good
            if (self.comparator)(&self.items[parent_idx], &self.items[idx]) {
                break;
            }
            // Swap parent and child
            self.items.swap(parent_idx, idx);
            idx = parent_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        // If no children, return idx (though this shouldn't be called in that case)
        if left_idx > self.count {
            return idx;
        }
        
        // If only left child exists
        if right_idx > self.count {
            return left_idx;
        }
        
        // Return the child that should come first according to comparator
        // For min-heap: return smaller child
        // For max-heap: return larger child
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        // Save the root value to return
        let root_value = std::mem::replace(&mut self.items[1], T::default());
        
        // Move the last element to the root
        self.items[1] = std::mem::replace(&mut self.items[self.count], T::default());
        self.count -= 1;
        
        // If heap is now empty, return the root
        if self.count == 0 {
            return Some(root_value);
        }
        
        // Bubble down to maintain heap property
        let mut idx = 1;
        while self.children_present(idx) {
            let smallest_child_idx = self.smallest_child_idx(idx);
            
            // If current node should come before its smallest child, we're done
            if (self.comparator)(&self.items[idx], &self.items[smallest_child_idx]) {
                break;
            }
            
            // Swap with smallest child
            self.items.swap(idx, smallest_child_idx);
            idx = smallest_child_idx;
        }
        
        Some(root_value)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}