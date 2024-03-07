use core::str;
use std::collections::{HashMap, VecDeque};

use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

struct LRUCache<K, V> {
    map: HashMap<K, Rc<RefCell<Node<(K, V)>>>>,
    head: Link<(K, V)>,
    tail: Link<(K, V)>,
    capacity: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

    fn get(&mut self, key: K) -> Option<V> {
        let node = self.map.get(&key).or_else(|| None).cloned();
        match node {
            None => None,
            Some(node) => {
                self.remove(node.clone());
                self.append(node.clone());
                Some(node.borrow().value.1.clone())
            }
        }
    }

    fn put(&mut self, key: K, value: V) {
        if let Some(node) = self.map.get(&key) {
            self.remove(node.clone());
        }
        if self.map.len() == self.capacity {
            if let Some(node) = self.head.clone() {
                self.remove(node.clone());
                self.map.remove(&node.borrow().value.0);
            }
        }
        let node = Rc::new(RefCell::new(Node {
            value: (key.clone(), value),
            prev: None,
            next: None,
        }));
        self.map.insert(key, node.clone());
        self.append(node);
    }

    fn remove(&mut self, node: Rc<RefCell<Node<(K, V)>>>) {
        if let Some(prev) = node.borrow().prev.clone() {
            prev.borrow_mut().next = node.borrow().next.clone();
        } else {
            self.head = node.borrow().next.clone();
        }
        if let Some(next) = node.borrow().next.clone() {
            next.borrow_mut().prev = node.borrow().prev.clone();
        } else {
            self.tail = node.borrow().prev.clone();
        }
    }

    fn append(&mut self, node: Rc<RefCell<Node<(K, V)>>>) {
        if let Some(tail) = self.tail.clone() {
            tail.borrow_mut().next = Some(node.clone());
            node.borrow_mut().prev = Some(tail);
            node.borrow_mut().next = None;
            self.tail = Some(node);
        } else {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
            node.borrow_mut().prev = None;
            node.borrow_mut().next = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(2);
        cache.put(1, "one");
        cache.put(2, "two");
        assert_eq!(cache.get(1), Some("one"));
        cache.put(3, "three");
        assert_eq!(cache.get(2), None);
        cache.put(4, "four");
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(3), Some("three"));
        assert_eq!(cache.get(4), Some("four"));
    }
}
