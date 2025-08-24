// An implementation of the Tree data structure in Rust

pub struct Tree<T> {
    head: Link<T>
}

type Link <T> = Option<Box<Node<T>>>;

struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    elem: T,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            left: self.head.take(),
            right: None,
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.left;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
}


#[cfg(test)]
mod test {
    use super::Tree;

    #[test]
    fn basics() {
        let mut tree: Tree<i32> = Tree::new();

        assert_eq!(tree.pop(), None);

        tree.push(1); tree.push(2); tree.push(3);
        assert_eq!(tree.peek(), Some(&3));
        assert_eq!(tree.pop(), Some(3));
        assert_eq!(tree.pop(), Some(2));
        assert_eq!(tree.pop(), Some(1));
    }
}
