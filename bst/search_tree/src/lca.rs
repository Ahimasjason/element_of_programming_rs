use super::*;

impl<T: Ord + Copy> Tree<T> {
    pub fn tree_lca<'a>(mut root: &Node<T>, n0: &'a Node<T>, n1: &'a Node<T>) -> Option<T> {
        while root.elem < n0.elem || root.elem > n1.elem {
            while root.elem < n0.elem {
                match root.right {
                    None => return None,
                    Some(ref n) => root = n,
                }
            }

            while root.elem > n1.elem {
                match root.left {
                    None => return None,
                    Some(ref n) => root = n,
                }
            }
        }
        Some(root.elem)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lca() {
        let mut tree = Tree::new();

        let elems = [
            19i32, 7, 3, 2, 5, 11, 17, 13, 43, 23, 37, 29, 31, 41, 47, 53,
        ];

        for i in elems {
            tree.insert(i);
        }
        let n0 = tree.get(29);
        let n1 = tree.get(41);
        let n2 = tree.get(53);
        println!("N0 {:?} , N1 {:?}", n0.unwrap(), n1.unwrap());

        assert_eq!(
            Tree::tree_lca(tree.root.as_ref().unwrap(), &n0.unwrap(), &n1.unwrap()),
            Some(37)
        );
        assert_eq!(
            Tree::tree_lca(tree.root.as_ref().unwrap(), &n0.unwrap(), &n2.unwrap()),
            Some(43)
        );
    }
}
