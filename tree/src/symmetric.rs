//! Binary tree is symmetic if let subtree is the mirror image of the right subtree
//! compute its mirror image of a tree and seeing if the mirror image is equall to the original tree
//! Computing the mirror image of a tree is as simple as swapping the left and right subtrees and recursively continuing

use super::{Node, Tree};

pub fn is_symmetric<T>(t: Tree<T>) -> bool
where
    T: std::cmp::Ord,
{
    match t.root {
        Some(r) => check_symmetric(r.left, r.right),
        None => true,
    }
}

fn check_symmetric<T>(subtree_0: Option<Box<Node<T>>>, subtree_1: Option<Box<Node<T>>>) -> bool
where
    T: std::cmp::Ord,
{
    match (subtree_0, subtree_1) {
        (None, None) => true,
        (Some(t0), Some(t1)) => {
            t0.val == t1.val
                && check_symmetric(t0.left, t1.right)
                && check_symmetric(t0.right, t1.left)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric() {
        // let mut t: Tree<usize> = Tree{
        //         root : Some(
        //                     Box::new(
        //                             Node{val: 314, left: Some(
        //                                 Box::new(Node {val: 6, left: None, right :
        //                                     Some(Box::new(Node{val: 2})
        //                                     }
        //                                     )
        //                             ) },
        // };

        assert!(true);
    }
}
