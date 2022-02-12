use super::*;

impl<T: Ord + Copy + std::fmt::Debug> Tree<T> {
    fn _build_from_preorder(preorder: &mut [T]) -> Option<Box<Node<T>>> {
        //  [43, 23, 37, 29, 31, 41, 47, 53]
        println!("preorder -> {:?}", preorder);

        if preorder.len() == 0 {
            return None;
        }
        let transition_point = {
            let mut point: Option<usize> = None;

            for i in 1..preorder.len() {
                if preorder[i] > preorder[0] {
                    point = Some(i);
                    break;
                }
            }

            match point {
                Some(n) => n,
                None => preorder.len(),
            }
        };

        println!("transistion point -> {:?}", transition_point);

        Some(Box::new(Node {
            elem: preorder[0],
            left: Tree::_build_from_preorder(&mut preorder[1..transition_point]),
            right: Tree::_build_from_preorder(&mut preorder[transition_point..]),
        }))
    }

    pub fn build_from_preorder(preorder: &mut Vec<T>) -> Tree<T> {
        Tree {
            root: Self::_build_from_preorder(preorder.as_mut_slice()),
        }
    }

    fn _collect_preoder(node: &Link<T>, res: &mut Vec<T>) -> Vec<T> {
        match node {
            Some(ref n) => {
                res.push(n.elem);
                Self::_collect_preoder(&n.left, res);
                Self::_collect_preoder(&n.right, res);
                res.to_vec()
            }
            None => res.to_vec(),
        }
    }

    fn _collect_inorder(node: &Link<T>, res: &mut Vec<T>) -> Vec<T> {
        match node {
            Some(ref n) => {
                Self::_collect_inorder(&n.left, res);
                res.push(n.elem);
                Self::_collect_inorder(&n.right, res);
                res.to_vec()
            }
            None => res.to_vec(),
        }
    }

    fn _build_preorder_with_state(
        preorder: &mut [T],
        state: &mut usize,
        min_val: T,
        max_val: T,
    ) -> Link<T> {
        if *state == preorder.len() {
            return None;
        }
        let root = preorder[*state];

        if !((min_val <= root) && (root <= max_val)) {
            return None;
        }

        *state += 1;

        let left_tree = Self::_build_preorder_with_state(preorder, state, min_val, root);
        let right_tree = Self::_build_preorder_with_state(preorder, state, root, max_val);

        Some(Box::new(Node {
            elem: root,
            left: left_tree,
            right: right_tree,
        }))
    }

    fn _build_from_inorder(inorder: &mut [T]) -> Option<Box<Node<T>>> {
        if !(inorder.len() > 0) {
            return None;
        }

        let len = inorder.len();
        let mid = len / 2;
        Some(Box::new(Node {
            elem: inorder[mid],
            left: Self::_build_from_inorder(&mut inorder[0..mid]),
            right: Self::_build_from_inorder(&mut inorder[(mid + 1)..len]),
        }))
    }

    fn build_from_inorder(inorder: &mut [T]) -> Tree<T> {
        Tree {
            root: Self::_build_from_inorder(inorder),
        }
    }

    fn build_preorder_with_state(preorder: &mut [T], min_val: T, max_val: T) -> Tree<T> {
        Tree {
            root: Self::_build_preorder_with_state(preorder, &mut (0 as usize), min_val, max_val),
        }
    }
    pub fn collect_preorder(&self) -> Vec<T> {
        Self::_collect_preoder(&self.root, &mut Vec::new())
    }

    pub fn collect_inorder(&self) -> Vec<T> {
        Self::_collect_inorder(&self.root, &mut Vec::new())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build_tree() {
        let mut preorder = [43, 23, 37, 29, 31, 41, 47, 53].to_vec();

        let mut tree = Tree::build_from_preorder(&mut preorder);

        // eprintln!(" tree builded => {:?}", tree);

        assert_eq!(preorder, tree.collect_preorder());
    }

    #[test]
    fn test_build_tree_with_state() {
        let mut preorder = [43i32, 23, 37, 29, 31, 41, 47, 53].to_vec();

        let mut tree = Tree::build_preorder_with_state(&mut preorder, std::i32::MIN, std::i32::MAX);

        // eprintln!(" tree builded => {:?}", tree);

        assert_eq!(preorder, tree.collect_preorder());
    }

    #[test]
    fn test_build_tree_with_inorder() {
        let mut inorder = [10i32, 12, 13, 14, 15, 16, 21, 24, 25].to_vec();

        let mut tree = Tree::build_from_inorder(&mut inorder);

        eprintln!(" tree builded inorder => {:?}", tree);

        assert_eq!(inorder, tree.collect_inorder());
    }
}
