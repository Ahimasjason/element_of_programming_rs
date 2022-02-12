use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub mod build_tree;
pub mod closet_element;
pub mod enumerate_sequence;
pub mod gr_than_k;
pub mod is_binary_tree;
pub mod k_th_largest;
pub mod lca;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
struct Node<T: Ord> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord> Node<T> {
    fn insert(&mut self, val: T) {
        if self.elem > val {
            match self.left {
                Some(ref mut left_node) => (**left_node).insert(val),
                None => {
                    self.left = Some(Box::new(Self {
                        elem: val,
                        left: None,
                        right: None,
                    }))
                }
            }
        } else {
            match self.right {
                Some(ref mut right_node) => (**right_node).insert(val),
                None => {
                    self.right = Some(Box::new(Self {
                        elem: val,
                        left: None,
                        right: None,
                    }))
                }
            }
        }
    }
}
#[derive(Debug, PartialEq)]
struct Tree<T: Ord> {
    root: Link<T>,
}

impl<T: Ord + std::fmt::Debug> Tree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, val: T) {
        match self.root {
            Some(ref mut node) => node.insert(val),
            _ => {
                self.root = Some(Box::new(Node {
                    elem: val,
                    left: None,
                    right: None,
                }))
            }
        }
    }

    fn _preorder(&self, node: &Link<T>) {
        match node {
            Some(ref n) => {
                println!("{:?}", n.elem);
                self._preorder(&n.left);
                self._preorder(&n.right);
            }
            None => {}
        }
    }

    fn preorder(&self) {
        self._preorder(&self.root);
    }

    // fn _get(node : &Link<T> , k: T) -> Option<&Node<T>>{

    //     match node {
    //         None => return None,
    //         Some(ref n) => {
    //                 if n.elem > k {
    //                     Self::_get(n.left, k);
    //                 }
    //         }

    //     }
    // }
    fn get(&self, k: T) -> Option<&Node<T>> {
        let mut curr_node = &self.root;

        while let Some(boxed_node) = curr_node {
            if boxed_node.elem > k {
                curr_node = &boxed_node.left;
            } else if boxed_node.elem < k {
                curr_node = &boxed_node.right;
            } else {
                break;
            }
        }

        curr_node.as_deref()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn get_works() {
        let mut tree = Tree::new();

        let elems = [10, 8, 9, 11, 6, 2];

        for i in elems {
            tree.insert(i);
        }
        // tree.preorder();
        // eprintln!("{:?}", tree.root);

        assert_eq!(tree.get(9).unwrap().elem, 9);
    }
}
