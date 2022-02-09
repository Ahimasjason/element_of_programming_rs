use std::mem;

// type TreeType<T> = Option<Box<Node<T>>>;
pub mod is_balanced;
pub mod symmetric;
pub mod lca;

#[derive(Debug)]
pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> Tree<T>
where
    T: std::cmp::Ord + std::fmt::Debug,
{
    pub fn new(val: T) -> Self {
        Self {
            root: Some(Box::new(Node::new(val))),
            length: 1,
        }
    }
    pub fn insert(&mut self, val: T) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_node(root, val);
    }

    fn add_node(&mut self, node: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
        match node {
            None => Some(Box::new(Node::new(val))),
            Some(mut n) => {
                if n.val <= val {
                    n.right = self.add_node(n.right, val);
                    Some(n)
                } else {
                    n.left = self.add_node(n.left, val);
                    Some(n)
                }
            }
        }
    }

    pub fn inorder_traversal(&self) {
        self.inorder(&self.root);
    }

    fn inorder(&self, node: &Option<Box<Node<T>>>) {
        if let Some(ref n) = node {
            // eprintln!("{:?}", n.val);
            self.inorder(&n.left);
            self.inorder(&n.right)
        }
    }

    pub fn post_traversal(&self) {
        self.postorder(&self.root);
    }

    fn postorder(&self, node: &Option<Box<Node<T>>>) {
        if let Some(ref n) = node {
            self.postorder(&n.right);
            self.postorder(&n.left);
            // eprintln!("{:?}", n.val);
        }
    }

    pub fn preorder_traversal(&self) {
        self.preorder(&self.root);
    }

    fn preorder(&self, node: &Option<Box<Node<T>>>) {
        if let Some(ref n) = node {
            self.preorder(&n.left);
            // eprintln!("{:?}", n.val);
            self.preorder(&n.right);
        }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: std::cmp::Ord,
{
    pub fn new(val: T) -> Self {
        Self {
            val: val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::prelude::*;

    #[test]
    fn test() {
        let t = Tree::new(10);

        println!("Tree ==> {:?} ", t);
        assert!(true);
    }

    #[test]
    fn test_tree() {
        let mut rng = rand::thread_rng();
        let mut v: Vec<usize> = (0..100).collect();
        for i in 0..v.len() {
            let r: usize = rng.gen_range(i..v.len());
            v.swap(i, r);
        }
        let mut t = Tree::new(v[0]);

        for i in 1..v.len() {
            t.insert(v[i]);
        }
        // println!("Tree ==> {:?} ", t);
        t.inorder_traversal();
        // eprintln!("Pre order --> ");
        t.preorder_traversal();
        // eprintln!("Post order traversal --> ");
        t.post_traversal();
        assert!(true);
    }

    #[test]
    fn test_insert() {
        let mut t = Tree::new(3);

        t.insert(2);
        t.insert(4);
        t.insert(3);

        eprintln!("{:?}", t);

        // match t.root {
        //     Some(e) => {eprintln!("match {:?}", e);},
        //     None => {},
        // }
        let b = t.root.unwrap();

        // eprintln!("B is {:?}", b);
        assert_eq!(b.right.unwrap().val, 4);
    }
}
