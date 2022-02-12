use crate::{Link, Node, Tree};
use std::collections::VecDeque;

impl<T: Ord + Copy + std::fmt::Debug> Tree<T> {
    fn _is_binary_tree(&self, node: &Link<T>, low_value: T, high_value: T) -> bool {
        println!("low_val {:?} high val {:?}", low_value, high_value);
        match node {
            None => true,
            Some(n) => {
                if !((low_value <= n.elem) && (n.elem <= high_value)) {
                    return false;
                }
                self._is_binary_tree(&n.left, low_value, n.elem)
                    && self._is_binary_tree(&n.right, n.elem, high_value)
            }
        }
    }
    fn is_binary_tree(&self, low_value: T, high_value: T) -> bool {
        self._is_binary_tree(&self.root, low_value, high_value)
    }

    fn is_binary_tree_deq(&self, low_value: T, high_value: T) -> bool {
        let mut deque = VecDeque::new();
        deque.push_back((&self.root, low_value, high_value));

        while let Some(node) = deque.pop_front() {
            match node.0 {
                Some(n) => {
                    if !((node.1 <= n.elem) && (n.elem <= node.2)) {
                        return false;
                    }

                    deque.push_back((&n.left, node.1, n.elem));
                    deque.push_back((&n.right, n.elem, node.2));
                }
                None => continue,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_binary_tree() {
        let mut tree = Tree::new();

        let elems = [10i32, 8, 9, 11, 6, 2];

        for i in elems {
            tree.insert(i);
        }

        assert!(tree.is_binary_tree(std::i32::MIN, std::i32::MAX));
    }

    #[test]
    fn test_is_not_binary_tree() {
        let mut tree: Tree<i32> = Tree {
            root: Some(Box::new(Node {
                elem: 10,
                left: Some(Box::new(Node {
                    elem: 12,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    elem: 9,
                    left: None,
                    right: None,
                })),
            })),
        };

        // let elems  = [10i32, 8, 9, 11, 6, 2];

        // for i in elems {
        //     tree.insert(i);
        // }

        eprintln!(
            "{:?}, {:?}",
            tree,
            tree.is_binary_tree(std::i32::MIN, std::i32::MAX)
        );
        assert!(!tree.is_binary_tree(std::i32::MIN, std::i32::MAX));
    }

    #[test]
    fn test_is_binary_tree_deq() {
        // let mut tree: Tree<i32> = Tree {
        //     root: Some(Box::new(Node {
        //         elem: 10,
        //         left: Some(Box::new(Node {
        //             elem: 12,
        //             left: None,
        //             right: None,
        //         })),
        //         right: Some(Box::new(Node {
        //             elem: 9,
        //             left: None,
        //             right: None,
        //         })),
        //     })),
        // };

        let mut tree: Tree<i32> = Tree {
            root: Some(Box::new(Node {
                elem: 10,
                left: Some(Box::new(Node {
                    elem: 12,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        };

        // let elems  = [10i32, 8, 9, 11, 6, 2];

        // for i in elems {
        //     tree.insert(i);
        // }

        eprintln!(
            "{:?}, {:?}",
            tree,
            tree.is_binary_tree(std::i32::MIN, std::i32::MAX)
        );
        assert!(!tree.is_binary_tree_deq(std::i32::MIN, std::i32::MAX));
    }
}
