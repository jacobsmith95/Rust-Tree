// An implementation of the Tree data structure in Rust

pub struct Tree<i32> {
    head: Link<i32>
}

type Link <i32> = Option<Box<Node<T>>>;

struct Node<i32> {
    left: Link<i32>,
    right: Link<i32>,
    elem: i32,
}

impl Tree<i32> {
    pub fn new() -> Self {
        Tree { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let cur_link = self.head.take();
        let new_node = Box::new(Node {
            elem: elem,
            left: if elem<cur_link.as_ref() {},
            right: if elem>cur_link.as_ref() {},
        });
        //self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.left;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&i32> {
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
