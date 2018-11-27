use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

pub struct NodeRef<T> {
    node: Rc<RefCell<Node<T>>>,
}

impl<T> NodeRef<T> {
    pub fn contents(&self) -> Ref<T> {
        Ref::map(self.node.borrow(), |node| &node.elem)
    }

    pub fn contents_mut(&mut self) -> RefMut<T> {
        RefMut::map(self.node.borrow_mut(), |node| &mut node.elem)
    }

    /// unwrap panics if it is borrowed someplace else
    pub fn unwrap(self) -> T {
        Rc::try_unwrap(self.node).ok().unwrap().into_inner().elem
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct IntoIter<T>(List<T>);

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    /// push_front gives you back a reference that you may use in the future to the node created in the list
    pub fn push_front(&mut self, elem: T) -> NodeRef<T> {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head.clone());
            }
            None => {
                self.head = Some(new_head.clone());
            }
        };
        NodeRef { node: new_head }
    }

    pub fn pop_front(&mut self) -> Option<NodeRef<T>> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.head = None;
                }
            }
            NodeRef { node: old_head }
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front().map(NodeRef::unwrap)
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use super::NodeRef;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front().map(NodeRef::unwrap), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front().map(NodeRef::unwrap), Some(3));
        assert_eq!(list.pop_front().map(NodeRef::unwrap), Some(2));

        // Push some more just to make sure nothing's corrupted
        {
            let _ref4 = list.push_front(4);
        }
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front().map(NodeRef::unwrap), Some(5));
        assert_eq!(list.pop_front().map(NodeRef::unwrap), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front().map(NodeRef::unwrap), Some(1));
        assert_eq!(list.pop_front().map(NodeRef::unwrap), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_front_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }
}
