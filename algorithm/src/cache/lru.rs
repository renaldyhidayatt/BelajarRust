use std::collections::HashMap;

struct LRUCache<'a> {
    dl: DoublyLinkedList<'a>,
    size: usize,
    capacity: usize,
    storage: HashMap<&'a str, Box<Node<'a>>>,
}

struct Node<'a> {
    key: &'a str,
    value: &'a str,
    prev: Option<&'a Node<'a>>,
    next: Option<&'a Node<'a>>,
}

struct DoublyLinkedList<'a> {
    head: Option<&'a Node<'a>>,
    tail: Option<&'a Node<'a>>,
}

impl<'a> LRUCache<'a> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            dl: DoublyLinkedList::new(),
            storage: HashMap::with_capacity(capacity),
            size: 0,
            capacity,
        }
    }

    fn get(&mut self, key: &str) -> Option<&str> {
        match self.storage.get(key) {
            Some(node) => {
                self.dl.move_to_back(node);
                Some(node.value)
            }
            None => None,
        }
    }

    fn put(&mut self, key: &'a str, value: &'a str) {
        match self.storage.get_mut(key) {
            Some(node) => {
                node.value = value;
                self.dl.move_to_back(node);
                return;
            }
            None => {
                if self.size >= self.capacity {
                    let node = self.dl.pop_front().unwrap();
                    self.storage.remove(node.key);
                    self.size -= 1;
                }
                let node = Node {
                    key,
                    value,
                    prev: None,
                    next: None,
                };
                let boxed_node = Box::new(node);
                self.dl.add_at_end(&boxed_node);
                self.storage.insert(key, boxed_node);
                self.size += 1;
            }
        }
    }
}

impl<'a> DoublyLinkedList<'a> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn add_at_end(&mut self, node: &'a Box<Node<'a>>) {
        match self.tail {
            Some(tail_node) => {
                node.prev = Some(tail_node);
                tail_node.next = Some(node);
                self.tail = Some(node);
            }
            None => {
                self.head = Some(node);
                self.tail = Some(node);
            }
        }
    }

    fn pop_front(&mut self) -> Option<&'a Node<'a>> {
        match self.head {
            Some(head_node) => {
                match head_node.next {
                    Some(next_node) => {
                        next_node.prev = None;
                        self.head = Some(next_node);
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                Some(head_node)
            }
            None => None,
        }
    }

    fn move_to_back(&mut self, node: &'a Box<Node<'a>>) {
        if self.tail == Some(node) {
            return;
        }

        match node.prev {
            Some(prev_node) => {
                prev_node.next = node.next;
            }
            None => {
                self.head = node.next;
            }
        }

        match node.next {
            Some(next_node) => {
                next_node.prev = node.prev;
            }
            None => {
                self.tail = Some(node);
            }
        }
        self.add_at_end(node);
    }
}
