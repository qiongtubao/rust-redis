use std::rc::Rc;

struct ListNode<T> {
    value: T,
    prev: Option<Rc<ListNode<T>>>,
    next: Option<Rc<ListNode<T>>>,
}
impl<T> ListNode<T> {
    pub fn new(value: T) -> Self {
        ListNode {
            prev: None,
            next: None,
            value
        }
    }
    fn get_next(&self) -> Option<Rc<Self>> {
        let mut t = self;
        if let Some(ref v) = t.next {
            return Some(Rc::clone(v));
        }
        None
    }

    fn get_next_mut(&mut self) -> Option<&mut Self> {
        let mut t = self;
        if let Some(ref mut v) = t.next {
            t = Rc::get_mut(v).unwrap();
            return Some(t);
        }
        None
    }
    fn get_prev(&mut self) -> Option<&mut Self> {
        let mut t = self;
        if let Some(ref mut v) = t.prev {
            t = Rc::get_mut(v).unwrap();
            return Some(t);
        }
        None
    }

}
pub struct List<T: Clone> {
    head: Option<Rc<ListNode<T>>>,
    tail: Option<Rc<ListNode<T>>>,
    len: u64,
}

impl<T> List<T> where T: Clone{
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            len : 0
        }
    }
    pub fn push(&mut self, data: T) {
        let mut node = ListNode::new(data);
        if let Some(ref mut v) = self.tail {
            node.prev = Some(Rc::clone(v));
            let c = Rc::new(node);
            let t = Rc::get_mut(v).unwrap();
            t.next = Some(Rc::clone(&c));
            self.tail = Some(c);
        } else {
            let c = Rc::new(node);
            self.head = Some(Rc::clone(&c));
        }

        self.len += 1;
    }

    pub fn get<'a>(&'a self, index: u64) -> Option<T> {
        if index > self.len {
            return None;
        }
        let mut i = 0;
        let mut t = self.head.clone();
        loop {
            if let Some(v ) = t {
                if i == index {
                    return Some(v.value.clone());
                }
                t = v.get_next();
            }else{
                return None;
            }
        }
        None
    }

}
