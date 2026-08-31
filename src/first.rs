use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty, 
    More(Box<Node>)
}

pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        List{head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32){
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        };
        self.head = Link::More(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head,  Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                let pop_item = node.elem;
                self.head = node.next;
                Some(pop_item)
            }
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
