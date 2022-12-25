use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
    //tail: Option<&'a mut Node<T>>
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn push(&mut self, elem: T) {
        unsafe {
            // immediately convert the `Box` into a raw pointer
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut()
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }
            self.tail = new_tail;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                // RISE FROM THE GRAVE
                let head = Box::from_raw(self.head);
                self.head = head.next;
                
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }
                Some(head.elem)
            }
        }
    }
    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.elem)
        }
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.elem)
        }
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter { next: self.head.as_ref() }
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut { next: self.head.as_mut() }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(0);
        list.push(1);
        assert_eq!(list.pop(), Some(0));
        list.push(2);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push(0);
        list.push(1);
        assert_eq!(list.pop(), Some(0));
        list.push(2);
        assert_eq!(list.peek(), Some(&1));
        if let Some(num) = list.peek_mut() {
            *num *= 10;
        }; 
        assert_eq!(list.peek(), Some(&10))

    }
}
