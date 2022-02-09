
use crate::{Tree, Node};


type TreeType<T> = Option<Box<Node<T>>>;

pub fn tree_lca<T>(t : Tree<T>, x : TreeType<T>, y: TreeType<T>) -> TreeType<T>
    where T: std::cmp::Ord,
{

    unimplemented!()
}


fn _lca<T>(t: TreeType<T>, node_0: TreeType<T>, node_1: TreeType<T>) -> (usize, Option<Box<Node<T>>>){


    match t {
        None => (0, None),
        Some(n) => {
            let left_res = _lca(n.left, node_0, node_1);
            if left_res.0 == 2 {
                return left_res;
            }

            let right_res = _lca(n.right, node_0, node_1);
            if right_res.0 == 2{
                return right_res;
            }


            let num_nodes = (left_res.0 + right_res.0);

            todo!() 
        },
    }
    unimplemented!()
}