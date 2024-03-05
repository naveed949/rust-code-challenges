use core::str;
use std::collections::{HashMap, VecDeque};

use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

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