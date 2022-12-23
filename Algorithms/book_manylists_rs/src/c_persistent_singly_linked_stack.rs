use std::rc::Rc;
//use std::sync::Arc;
// for thread safe, just change each `Rc` to `Arc`

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List { head: None }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn prepend(&self, elem: T) -> Self {
        List { head: Some(Rc::new(Node {
            elem,
            next: self.head.clone()
        }))}
    }
    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node|
                node.next.clone()
            )
        }
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            let Ok(mut node) = Rc::try_unwrap(node) else {
                break;
            };
            head = node.next.take();
        }
    }
}
