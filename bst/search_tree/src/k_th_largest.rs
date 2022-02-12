use super::*;

impl<T: Ord + Copy> Tree<T> {
    pub fn _compute(tree: Option<&Node<T>>, elements: &mut Vec<T>, k: usize) {
        match tree {
            Some(ref node) => {
                Self::_compute(node.right.as_deref(), elements, k);
                if elements.len() < k {
                    elements.push(node.elem);
                    if elements.len() < k {
                        Self::_compute(node.left.as_deref(), elements, k);
                    }
                }
            }
            None => {}
        }
    }
    pub fn k_largest(&self, k: usize) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();
        Self::_compute(self.root.as_deref(), &mut vec, k);

        vec
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

        assert_eq!(tree.k_largest(3), [53, 47, 43]);
        // println!("{:?}", tree);
    }
}

/*
        Tree{ root: Some(Node { elem: 19,
        left:Some(Node { elem: 7,
                        left: Some(Node { elem: 3,
                                        left: Some(Node { elem: 2,
                                                            left: None, right: None }),
                                        right: Some(Node { elem: 5, left: None, right: None }) }),
                        right: Some(Node { elem: 11,
                                            left: None,
                                            right: Some(Node { elem: 17,
                                                                left: Some(Node { elem: 13,
                                                                                    left: None,
                                                                                    right: None }),
                                                                right: None }) }) }),
        right: Some(Node { elem: 43,
                            left: Some(Node { elem: 23,
                                                left: None,
                                                right: Some(Node { elem: 37,
                                                                    left: Some(Node { elem: 29,
                                                                                        left: None,
                                                                                        right: Some(Node { elem: 31,
                                                                                                            left: None,
                                                                                                            right: None })
                                                                                    }),
                                                                    right: Some(Node { elem: 41,
                                                                                        left: None,
                                                                                        right: None })
                                                                }) }),
                            right: Some(Node { elem: 47,
                                                left: None,
                                                right: Some(Node { elem: 53, left: None, right: None }) }) }) }) }

*/
