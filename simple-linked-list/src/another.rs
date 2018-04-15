use std::mem;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    fn iter(&self) -> SimpleLinkedListIterator<T> {
        SimpleLinkedListIterator { next: self.head.as_ref().map(|bx| &**bx) }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: mem::replace(&mut self.head, None),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|bx| {
            let node = *bx;
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|bx| &bx.data)
    }
}

struct SimpleLinkedListIterator<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for SimpleLinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|bx| {
            self.next = bx.next.as_ref().map(|bx| &**bx);
            &bx.data
        })
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        for item in self.iter() {
            reversed.push(item.clone())
        }
        reversed
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut other = Self::new();
        for value in item {
            other.push(value.clone())
        }
        other
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut values = vec![];
        while let Some(value) = self.pop() {
            values.push(value);
        }
        values.reverse();
        values
    }
}
