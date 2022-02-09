// use std::collections::BTreeSet;
// use std::hash::Hash;
// use std::rc::Rc;
// use std::cell::{RefCell};

// type BareTree <T: Copy> = Rc<Node<T>>;
// type TreeNode <T : Copy> = Option<BareTree<T>>;
// // type TreeNodeRef <T> = Option<&BareTree<T>>;

// #[derive(Debug,PartialEq, Eq, PartialOrd, Ord, Clone)]
// struct Node<T: Copy>{
//     data : T,
//     left : TreeNode<T>,
//     right : TreeNode<T>,
//     parent : TreeNode<T>,
// }




// fn find_lca<'a , T: Ord+ Copy>(mut node1 : Option<&'a BareTree<T>>, mut  node2 : Option<&'a BareTree<T>>) -> Option<&'a BareTree<T>> 
// {

//     let mut set = BTreeSet::new();

//     while node1.is_some() || node2.is_some(){
//         if node1.is_some(){
//             if set.contains(&node1){
//                 // return node1;
//                 eprintln!("Found the node");
//                 return node1
//             }
//             set.insert(node1);
//             node1 = node1.take().unwrap().parent.as_ref();
//         }

//         if node2.is_some(){
//             if set.contains(&node2){
//                 // return node2;
//                 eprintln!("Found the node");
//                 return node2
//             }
//             set.insert(node2);
//             node2 = node2.take().unwrap().parent.as_ref();
//         }
//     }

//     None
// }



// #[cfg(test)]

// mod tests {

//     use super::*;



//     fn build<T> (iter : &mut T, parent : TreeNode<isize>) -> TreeNode<isize>
//         where 
//             T: Iterator<Item = Option<isize>>,
//     {

//         let item = iter.next().unwrap();
//         if item.is_none(){
//             return None
//         }

//         let mut new = Rc::new(
//                 // RefCell::new(
//                         Node {
//                                 data : item.unwrap(),
//                                 left : None,
//                                 right: None,
//                                 parent : parent
//                         }
//                 // )
//                 )
//         ;
        

        
//         new.left = build(iter, Some(Rc::clone(&new)));
//         new.right = build(iter, Some(Rc::clone(&new)));
//         Some(new)
//     }




//     fn build_p_tree() ->  TreeNode<isize>{

//         let mut vals  = [
//            Some(1isize),Some(2), Some(4),None,None,Some(6),Some(8),None,None,Some(7),None,None,Some(3),None,Some(5),None,None
//         ];

//         let r = build(&mut vals.into_iter(), None);
//         r
//     }

//     #[test]
//     fn do_test(){

//         let r = build_p_tree();
//         eprintln!("{:?}", r);

//         assert!(true);
//     }
// }