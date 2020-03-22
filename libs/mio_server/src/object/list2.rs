use std::sync::Arc;

struct ListNode<T> {
    prev: Option<Box<ListNode<T>>>,
    next: Option<Box<ListNode<T>>>,
    value: T
}
impl<T> ListNode<T> {
    pub fn new(value: T) -> ListNode<T> {
        ListNode {
            prev: None,
            next: None,
            value
        }
    }
    fn set_next(&mut self, node: Self) {
        self.next = Some(Box::new(node));
    }
    fn get_last<'a>(&'a mut self) -> &'a mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }
    fn push(&mut self, elem: T) {
        let new_node = ListNode::new(elem);
        new_node
        self.get_last().set_next(new_node);
    }
}
const AL_START_HEAD: u8 = 0;
const AL_START_TAIL: u8 = 1;
struct ListIter<T> {
    next_node: Option<Arc<ListNode<T>>>,
    direction: u8,
}
impl<T> ListIter<T> {
    pub fn new(next_node: Option<Arc<ListNode<T>>>, direction: u8) -> ListIter<T> {
        ListIter {
            next_node,
            direction
        }
    }
    pub fn next(&mut self) -> Option<Arc<ListNode<T>>> {
        match &self.next_node {
            Some(x) => {
                let next = self.next_node.clone();
                if self.direction == AL_START_HEAD {
                    self.next_node = x.next.clone();
                } else {
                    self.next_node = x.prev.clone();
                }
                next
            }
            None => {
                None
            }
        }
    }
}

pub struct List<T> {
    head: Option<Arc<ListNode<T>>>,
    tail: Option<Arc<ListNode<T>>>,
    len: u32,
}
impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
            len: 0
        }
    }
    pub fn empty(&mut self) {
        let mut current = self.head.clone();
        loop {
            match current {
                Some(data) => {
                    current = data.next.clone();
                }
                None => break
            }
        }
        self.len = 0;
        self.head = None;
        self.tail = None;
    }
    pub fn add_head(&mut self , value: T) {
        let mut node = ListNode::new(value);
        if self.len == 0 {
            self.head = Some(Arc::new(node));
            self.tail = self.head.clone();

        } else {
            node.next = self.head.clone();
            if let Some(head) = &self.head {
                head.next = self.head.clone();
                self.head = Some(Arc::new(node));
            }
        }
        self.len += 1;
    }
    pub fn add_tail(&mut self, value: T) {
        let mut node = ListNode::new(value);
        if self.len == 0 {
            self.head = Some(Arc::new(node));
            self.tail = self.head.clone();
        } else {
            node.prev = self.tail.clone();
            if let Some(tail) = &self.tail {
                tail.next = Some(Arc::new(node));
                self.tail = tail.next.clone();
            }
        }
        self.len += 1;
    }
    pub fn del_node(&mut self, node: &mut Arc<ListNode<T>>) {
        match &node.prev {
            Some(prev) => {
                prev.next = node.next.clone();
            }
            None => {
                self.head = node.next.clone();
            }
        }
        match &node.next {
            Some(next) => {
                next.prev = node.prev.clone();
            }
            None => {
                self.tail = node.prev.clone();
            }
        }
        self.len -= 1;
    }
    pub fn listGetIterator(&mut self, direction: u8) -> ListIter<T>{
        if direction == AL_START_HEAD {
            return ListIter::new(self.head.clone(), direction);
        }else{
            return ListIter::new(self.tail.clone(), direction);
        }
    }
    pub fn get(&mut self, index: u64) -> Option<Arc<ListNode<T>>> {
        let mut iter = self.listGetIterator(0);
        let mut i = 0;
        loop {
            if i == index {
                return iter.next();
            }
            iter.next();
            i+=1;

        }
    }
}