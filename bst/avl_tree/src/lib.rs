type TreeType = Option<Box<Node>>;

#[derive(Debug)]
struct AvlTree {
    root: TreeType,
    length: usize,
}

impl AvlTree {
    pub fn new() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }
    pub fn insert(&mut self, val: usize) {
        match self.root {
            Some(ref mut n) => {
                n.insert(val);
            }
            None => self.root = Some(Box::new(Node::new(val))),
        }
    }

    // fn height(node : TreeType ) -> usize{

    //     if node.is_none(){
    //         return 0;
    //     } else {
    //         let n = node.unwrap();
    //         let left_height = Self::height(n.left);
    //         let right_height = Self::height(n.right);

    //         if left_height > right_height {
    //             left_height + 1
    //         } else {
    //             right_height + 1
    //         }
    //     }
    // }

    fn height(node: TreeType) -> usize {
        if node.is_none() {
            0
        } else {
            match node {
                Some(n) => {
                    let left_height = match n.left {
                        Some(l) => l.height,
                        None => 0,
                    };
                    let right_height = match n.right {
                        Some(r) => r.height,
                        None => 0,
                    };
                    if left_height > right_height {
                        left_height + 1
                    } else {
                        right_height + 1
                    }
                }
                None => 0,
            }
        }
    }
}
#[derive(Debug)]
struct Node {
    data: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    height: usize,
}

impl Node {
    pub fn new(val: usize) -> Self {
        Self {
            data: val,
            left: None,
            right: None,
            height: 1,
        }
    }

    fn height(&self) -> usize {
        let left_height = match self.left {
            Some(ref l) => l.height,
            None => 0,
        };
        let right_height = match self.right {
            Some(ref r) => r.height,
            None => 0,
        };
        if left_height > right_height {
            left_height + 1
        } else {
            right_height + 1
        }
    }

    pub fn insert(&mut self, val: usize) {
        if self.data > val {
            match self.left {
                None => self.left = Some(Box::new(Self::new(val))),
                Some(ref mut n) => {
                    n.insert(val);
                }
            }
        } else {
            match self.right {
                None => self.right = Some(Box::new(Self::new(val))),
                Some(ref mut n) => n.insert(val),
            }
        }
        self.height = self.height();
        eprintln!(
            "{:?} => balance_factor {:?} on inserting {:?}",
            self.data,
            self.balance_factor(),
            val
        );
        // Self {data: val, left:None, right: None, height: 1}
    }

    fn balance_factor(&self) -> isize {
        let hl = match self.left {
            Some(ref l) => l.height,
            None => 0,
        };

        let hr = match self.right {
            Some(ref r) => r.height,
            None => 0,
        };

        (hl - hr) as isize
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_avl() {
        let mut avl = AvlTree::new();
        avl.insert(30);
        avl.insert(20);
        avl.insert(40);
        avl.insert(10);
        avl.insert(5);
        println!("{:?}", avl);
    }
}
