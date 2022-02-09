

use std::borrow::Borrow;

use super::*;




fn merge_sorted_list<T: Default + Ord>( mut a:Link<T>, mut b: Link<T>) -> Link <T>{
    

    let mut dummy = Some(Rc::new(RefCell::new(Node{data: T::default() , next: None})));
    let mut tail = dummy.clone();

    while a.is_some() && b.is_some(){

        let mut a_node = a.take();
        let mut b_node = b.take();

        if (*a_node.as_ref().unwrap().borrow_mut()).data < (*b_node.as_ref().unwrap().borrow_mut()).data{
            (*tail.as_ref().unwrap().borrow_mut()).next = a.clone();
            // a = (*a_data).next;
            let next_a = a_node.take().unwrap().borrow_mut().next.take();
            a = next_a;
            b = b_node;

        } else {
            (*tail.as_ref().unwrap().borrow_mut()).next = b.clone();
            // a = (*a_data).next;
            let next_b = b_node.take().unwrap().borrow_mut().next.take();
            b = next_b;
            a = a_node;
        }

        let nxt = tail.as_mut().map(|n|{
            let nxt = n.borrow_mut().next.clone();
            nxt
        });

        tail =nxt.unwrap();
        

    }

    // let d:Link<T>  = dummy.take().unwrap().borrow_mut().next;
    // d.clone();
    // None
    dummy
}





#[test]
fn test_merge_sorted_list(){

    let mut a_list = List::new();
    for i in [1,3,5,7,9]{
        a_list.insert(i);
    }

    let mut b_list = List::new();
    for i in [2,4,6,8]{
        b_list.insert(i);
    }
    merge_sorted_list(a_list.head, b_list.head);

}