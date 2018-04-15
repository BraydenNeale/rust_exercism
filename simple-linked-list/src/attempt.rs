pub struct SimpleLinkedList<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: self.head.take(),
        });

        self.len += 1;
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.len -= 1;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.element
        })
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        unimplemented!()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {

    fn into(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);

        while let Some(mut node) = self.head.take() {
            self.head = node.next.take();
            v.push(node.element)
        }

        v.reverse();
        v
    }
}
