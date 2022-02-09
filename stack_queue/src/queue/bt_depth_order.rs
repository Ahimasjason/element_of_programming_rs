


use std::collections::HashMap;
use std::process::Output;
use std::rc::Rc;
use std::cell::RefCell;




#[derive(Debug)]
struct Node<T>{
    data : T,
    left :Option<Box<Node<T>>>,
    right : Option<Box<Node<T>>>,
}






fn binary_tree_depth_order<T:Copy>(root: &Option<Box<Node<T>>>) -> Vec<Vec<T>>{

    let mut res = vec![];
    if root.is_none(){return res}
    let mut queue = Vec::new();

    queue.push(root.as_ref());
    while !queue.is_empty(){

        
        let data = queue
                            .iter()
                            .filter(|n| n.is_some())
                            .map(|m| m.as_ref().unwrap().data).collect::<Vec<T>>();
        
        // for node in &queue{
        //     if node.is_some(){
        //         data.push(node.unwrap().data);
        //     }
        // }
        if !data.is_empty(){

            res.push(data);
        }

        let mut new = vec![];
        for node in queue.drain(..){
            if node.is_some(){
                new.push(node.as_ref().unwrap().left.as_ref());
                new.push(node.as_ref().unwrap().right.as_ref());
            }
        }
        queue = new;

    }
   res
}


fn zigzag_traversal<T:Copy>(root: &Option<Box<Node<T>>>) -> Vec<Vec<T>>{

    
    let mut state = 0;
    let mut res = vec![];
    if root.is_none(){return res}
    let mut stack = Vec::new();
    
    stack.push(root.as_ref());
    
    while !stack.is_empty(){
        let mut data = vec![];
        let mut new = vec![];
        
        while !stack.is_empty(){
            let node = stack.pop().unwrap();
            if node.is_some(){
                data.push(node.as_ref().unwrap().data);
                if state %2 == 1{
                    new.push(node.as_ref().unwrap().right.as_ref());
                    new.push(node.as_ref().unwrap().left.as_ref());
                } else {
                    new.push(node.as_ref().unwrap().left.as_ref());
                    new.push(node.as_ref().unwrap().right.as_ref());
                }
            }
        }

        state += 1;
        stack = new;
        if !data.is_empty(){
            res.push(data);
        }
    }

    res
}






















fn _build_from_pre_in(preorder: &Vec<usize>,
                    inorder : &HashMap<usize,usize>,
                    inorder_start: usize,
                    inorder_end : usize,
                    pre_index :  Rc<RefCell<usize>>
) -> Option<Box<Node<usize>>>
{
        if inorder_start > inorder_end {
            return None
        }
        
        let p_index = {
            *pre_index.borrow()
        };

        if p_index >= preorder.len(){
            return None
        }

        eprintln!("pindex  {:?}", pre_index);
        let mut node = Node {data: preorder[p_index],left:None, right:None};

        let idx = inorder.get(&preorder[p_index]).unwrap();
        
        *pre_index.borrow_mut() = p_index + 1;
        if inorder_start == inorder_end {
            return Some(Box::new(node));
        }
        node.left = _build_from_pre_in(preorder, inorder, inorder_start, idx -1, Rc::clone(&pre_index));
        
        node.right = _build_from_pre_in(
                            preorder, 
                            inorder,
                             idx+1, 
                             inorder_end,
                              Rc::clone(&pre_index)
        );

        
        Some(
            Box::new(node)
        )
        

}


fn build_from_pre_in(preorder: Vec<usize> , inorder: Vec<usize>) -> Option<Box<Node<usize>>>{

    let hm : HashMap<usize,usize> = inorder
                                    .into_iter()
                                    .enumerate()
                                    .map(|v| (v.1, v.0)).collect(); 
    eprintln!("hm {:?}", hm);
    _build_from_pre_in(
            &preorder,
            &hm,
            0,
            preorder.len() - 1,
            Rc::new(RefCell::new(0))
    )

}



fn _build_from_none<T : core::iter::Iterator<Item = Option<usize>>>(iter: &mut T) -> Option<Box<Node<usize>>>{

    let n = iter.next().unwrap();

    if n.is_none(){return None}

    Some(Box::new(Node{data:n.unwrap(), left : _build_from_none(iter), right : _build_from_none(iter)}))
}




fn build_from_none(order : Vec<Option<usize>>) -> Option<Box<Node<usize>>>{
    _build_from_none(&mut order.into_iter())
}




#[cfg(test)]
mod tests{

    use super::*;


    #[test]
    fn test_build(){

        let pre =     vec![314,6,271,28,0,561,3,17,6,2,1,401,641,257,271,28];

        let inorder = vec![28,271,0,6,561,17,3,314,2,401,641,1,257,6,271,28];
    
        assert!(pre.len() == inorder.len());
    }


    #[test]
    fn test_build_non_dup(){

        let pre = vec![1,2,4,3,5];
        let inorder = vec![4,2,1,5,3];

        let r = build_from_pre_in(pre, inorder);
        eprintln!("{:?}", r);
    }

    
    fn test_build_from_none() -> Option<Box<Node<usize>>>{

        // let pre =     vec![314,6,271,28,0,561,3,17,6,2,1,401,641,257,271,28];
        let pre = vec![
            Some(314),Some(6),Some(217),Some(28),None,None,Some(0),None,None,Some(561),
            None,Some(3),Some(17),None,None,None,Some(6),Some(2),None,Some(1),Some(401),None,Some(641),
            None,None,Some(257), None, None, Some(271),None,Some(28),None,None 
        ];

        let t = build_from_none(pre);
        t
    }


    #[test]
    fn test_depth_order(){
        let t = test_build_from_none();
        let res = binary_tree_depth_order(&t);

        assert_eq!(vec![vec![314], vec![6, 6], vec![217, 561, 2, 271], vec![28, 0, 3, 1, 28], vec![17, 401, 257], vec![641]], res)

    }

    #[test]
    fn test_zig_zag(){
        let t = test_build_from_none();
        let res = zigzag_traversal(&t);
        assert_eq!(res, vec![vec![314], vec![6, 6], vec![217, 561, 2, 271], vec![28, 1, 3, 0, 28], vec![17, 401, 257], vec![641]]);
    }
}