struct Node<T: Default> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Default)]
pub struct List<T: Default> {
    head: Link<T>,
}

impl<T: Default> List<T> {
    pub fn new() -> Self {
        List::default()
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_node| &boxed_node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|boxed_node| &mut boxed_node.elem)
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

pub struct IntoIter<T: Default>(List<T>);

impl<T: Default> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T: Default> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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
    }

    #[test]
    fn test_pop() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.peek(), Some(&1));

        list.push(2);
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.peek_mut(), Some(&mut 1));
        if let Some(node) = list.peek_mut() {
            *node = 10;
        }
        assert_eq!(list.peek(), Some(&10));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut list_iter = list.into_iter();
        assert_eq!(list_iter.next(), Some(3));
        assert_eq!(list_iter.next(), Some(2));
        assert_eq!(list_iter.next(), Some(1));
    }
}
