struct Node {
    elem: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[derive(Default)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List::default()
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_push() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }
}
