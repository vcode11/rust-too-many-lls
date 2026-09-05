use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl Default for List {
    fn default() -> Self {
        List { head: Link::Empty }
    }
}

impl List {
    pub fn new() -> Self {
        List::default()
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                let pop_item = node.elem;
                self.head = node.next;
                Some(pop_item)
            }
        }
    }
}

// A non recurisve drop Impl
impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = curr_link {
            curr_link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_push() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
    }
}
