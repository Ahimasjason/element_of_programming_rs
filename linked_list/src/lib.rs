
use std::rc::Rc;
use std::cell::{
        RefCell,
        Ref,
};

mod merge_sorted_list;

// type Link<T> = Option<Box<Node<T>>>;

// type LinkRef<T>  = Box<Node<T>>;


// #[derive(Debug,Clone)]
// struct Node<T> {
    
//     data: T,
//     next: Link<T>
// }

// impl <T> Node <T> {

//     fn new(data: T) -> LinkRef<T> {
//         Box::new(Self{data, next: None})
//     }
// }



// #[derive(Debug)]
// struct List<T>{
//     head : Link<T>,
//     tail : Link<T>
// }

// impl <T> List <T> 

// where T : Ord + PartialEq 
// {

//     fn new() -> Self {
//         Self{head:None, tail:None}
//     }

//     fn insert(&mut self, data: T) -> () {
        
//         let new = Node::new(data);
//         match self.tail.as_mut().take() {

//             Some(node) => node.next = Some(new),
//             None => {
//                 self.head = Some(new.clone());
//             }
//         }
//         self.tail = Some(new);
//         ()
//     }

//     // fn search(&self, key:T) -> Option<T>{

//     //     let mut curr_node = self.head.as_ref();

//     //     while let Some( node) = curr_node{

//     //             let n = {
//     //                 &node.borrow().data
//     //             };
//     //             if *n == key {
//     //                 // return Some(node.borrow())
//     //                 return None
//     //             }
//     //             // curr_node = node.borrow().next.as_ref();
//     //             let c = &node.borrow().next;
//     //             curr_node = c.as_ref();
                
                
//     //     }
//     //     None
//     // }
// }



type Link<T> = Option<Rc<RefCell<Node<T>>>>;

type LinkRef<T>  = Rc<RefCell<Node<T>>>;


#[derive(Debug)]
struct Node<T> {
    
    data: T,
    next: Link<T>
}

impl <T> Node <T> {

    fn new(data: T) -> LinkRef<T> {
        Rc::new(
            RefCell::new(Self{data, next: None})
        )
    }
}



#[derive(Debug)]
struct List<T>{
    head : Link<T>,
    tail : Link<T>
}

impl <T> List <T> 

where T : Ord + PartialEq 
{

    fn new() -> Self {
        Self{head:None, tail:None}
    }

    fn insert(&mut self, data: T) -> () {
        
        let new = Node::new(data);
        match self.tail.as_mut().take() {

            Some(node) => node.borrow_mut().next = Some(new.clone()),
            None => {
                self.head = Some(new.clone());
            }
        }
        self.tail = Some(new);
        ()
    }

    // fn search(&self, key:T) -> Option<T>{

    //     let mut curr_node = self.head.as_ref();

    //     while let Some( node) = curr_node{

    //             let n = {
    //                 &node.borrow().data
    //             };
    //             if *n == key {
    //                 // return Some(node.borrow())
    //                 return None
    //             }
    //             // curr_node = node.borrow().next.as_ref();
    //             let c = &node.borrow().next;
    //             curr_node = c.as_ref();
                
                
    //     }
    //     None
    // }
}






#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_insert() {
        let mut ll = List::new();

        for i in 1..10{
            let i = ll.insert(i);
            assert_eq!(i,());
        }
        eprintln!("{:?}",ll);

        
    }


    // #[test]
    // fn test_search() {
    //     let mut ll = List::new();

    //     for i in 1..10{
    //         let i = ll.insert(i);
    //         assert_eq!(i,());
    //     }
    //     let search_elem = ll.search(5);
        
    //     eprintln!(" Search element {:?}", search_elem);
    //     assert!(search_elem.is_some());
    //     assert!(ll.search(20).is_none());

        
    // }
}
