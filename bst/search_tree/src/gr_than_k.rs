use super::*;

impl<T: Ord + Copy> Tree<T> {
    pub fn first_gr_than_k(&self, k: T) -> Option<T> {
        let mut subtree = &self.root;
        let mut first = &None;

        while let Some(ref node) = subtree {
            if node.elem > k {
                first = subtree;
                subtree = &node.left;
            } else {
                subtree = &node.right;
            }
        }

        first.as_deref().map(|n| n.elem)
    }

    fn _first_equal(n: Option<&Node<T>>, k: T) -> Option<&Node<T>> {
        if n.is_none() {
            return None;
        }

        let left_res = Self::_first_equal(n.unwrap().left.as_deref(), k);
        let right_res = Self::_first_equal(n.unwrap().right.as_deref(), k);
        if left_res.is_some() {
            return left_res;
        } else if n.unwrap().elem == k {
            return n;
        } else if right_res.is_some() {
            return right_res;
        } else {
            return None;
        }
    }

    pub fn first_equal_to_k(&self, k: T) -> Option<&Node<T>> {
        Self::_first_equal(self.root.as_deref(), k)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gr_k() {
        let mut tree = Tree::new();

        let elems = [
            19i32, 7, 3, 2, 5, 11, 17, 13, 43, 23, 37, 29, 31, 41, 47, 53,
        ];

        for i in elems {
            tree.insert(i);
        }

        // println!("{:?}", tree);

        assert_eq!(29, tree.first_gr_than_k(23).unwrap());
    }

    #[test]
    fn test_eq_k() {
        let mut tree = Tree::new();

        let elems = [108i32, 108, -10, 108, -14, 2, 285, 243, 285, 401];

        for i in elems {
            tree.insert(i);
        }
        println!("pre {:?}", tree);
        tree.preorder();
        println!(
            "*********  First equal {:?} End Here *******",
            tree.first_equal_to_k(108)
        );

        // assert_eq!(29, tree.first_gr_than_k(23).unwrap());
    }
}
