pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            length: 0,
        }
    }

    /// O(1)
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        let new_node = Node::new(element, self.head.take());

        self.head = Some(Box::new(new_node));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.length -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();

        let mut next = &self.head;
        while let &Some(ref node) = next {
            result.push(node.data.clone());
            next = &node.next;
        }

        result
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list: SimpleLinkedList<T> = SimpleLinkedList::new();
        for i in item {
            list.push(i.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut result_vec: Vec<T> = vec![];
        while self.len() > 0 {
            result_vec.push(self.pop().unwrap());
        }
        result_vec.reverse();
        result_vec
    }
}
