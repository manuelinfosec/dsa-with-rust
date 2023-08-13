use std::collections::{HashMap, LinkedList};
use std::hash::Hash;
use std::time::{Duration, Instant};

struct LRUCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    cache: HashMap<K, (V, Instant)>, // associates keys (of type K) with values (a tuple containing the value of type V and a timestamp of when it was inserted).
    order: LinkedList<K>, // keeps track of the order in which keys were most recently accessed.
    capacity: usize,      // maximum number of items the cache can hold.
    max_age: Duration,    // maximum amount of time an item can stay in the cache.
}

impl<K: Eq + Clone + Hash, V: Clone> LRUCache<K, V> {
    fn new(capacity: usize, max_age: Duration) -> LRUCache<K, V> {
        LRUCache {
            // create an empty hashmap
            cache: HashMap::new(),
            // create an empty linkedlist
            order: LinkedList::new(),
            capacity,
            max_age,
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        // collect value from the cache
        if let Some((value, _)) = self.cache.get(key) {
            // update order of the cache by pushing the latest from the front
            self.order.push_front(key.clone());
            // return the value
            Some(value)
        } else {
            // key not found in the cache
            None
        }
    }

    fn insert(&mut self, key: K, value: V) {
        // insert the new item to the cache
        self.cache.insert(key.clone(), (value, Instant::now()));
        // update the order of the linkedlist
        self.order.push_front(key);

        // Remove oldest item if the cache is full
        if self.cache.len() > self.capacity {
            // find the oldest key in the linked list
            if let Some(oldest_key) = self.order.pop_back() {
                self.cache.remove(&oldest_key);
            }
        }

        // remove exipired items
        let now = Instant::now();
        let mut new_order = LinkedList::new();
        // iterate through the linked list
        for key in self.order.iter() {
            // collect the current key from the cache
            if let Some((_, timestamp)) = self.cache.get(&key) {
                // determine how long the current value has been in the cache
                if now - *timestamp <= self.max_age {
                    // push the key to the back of the double linkedlist
                    new_order.push_back(key.clone());
                }
            }
        }
        // update the LRUCache order
        self.order = new_order;
    }
}

fn main() {}
