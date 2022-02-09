use crate::{Node, Tree};

pub fn check_balanced<T>(t: Tree<T>) -> bool {
    let h = is_balanced(t.root);
    eprintln!("res ==> {:?}", h);
    h.0
}

fn is_balanced<T>(tree_node: Option<Box<Node<T>>>) -> (bool, isize) {
    match tree_node {
        None => (true, -1),
        Some(n) => {
            let left_res = is_balanced(n.left);
            if !left_res.0 {
                return (false, 0);
            }
            let right_res = is_balanced(n.right);
            if !right_res.0 {
                return (false, 0);
            }

            let balaced: bool = (left_res.1 - right_res.1).abs() <= 1;

            let max_height = (left_res.1).max(right_res.1) + 1;
            (balaced, max_height)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_balanced() {
        println!("calling here");
        let mut t = Tree::new(3);
        t.insert(2);
        t.insert(4);
        assert!(check_balanced(t));
        assert!(true);
    }

    #[test]
    fn test_balanced_false() {
        println!("calling here");
        let mut t = Tree::new(3);
        t.insert(2);
        t.insert(4);
        t.insert(7);
        t.insert(8);
        // t.insert(1);
        // t.insert(0);
        assert!(!check_balanced(t));
        // assert!(true);
    }
}
