pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<Box<LinkedList<T>>>,
}

impl LinkedList<i32> {
    /// Creates and empty LinkedList that may hold i32 values
    ///
    /// Example:
    /// ```
    /// let list = mini_linked_list::LinkedList::<i32>::new();
    /// ```
    pub fn new() -> LinkedList<i32> {
        LinkedList {
            val: None,
            next: None,
        }
    }

    pub fn push_left(mut self, x: i32) -> LinkedList<i32> {
        let node = LinkedList::<i32> {
            val: Some(x),
            next: Some(Box::new(self)),
        };

        node
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_push_left() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list = list.push_left(1);
        list = list.push_left(2);
        list = list.push_left(3);
        list.push_left(4);
        // assert_eq!(list.collect(), vec![1, 2, 3, 4]);
    }
}
