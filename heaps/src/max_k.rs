//! Given a max heap represented as an array
//! compute largest k element in an array without modifying the heap
//! do not use pop() 0(k log n) 


use std::collections::BinaryHeap;




pub fn max_k<T: Ord + Copy> (arr : Vec<T> , k : usize) -> Vec<T> 

{
    let mut res = vec![];
    if arr.is_empty() {return res}
    let mut max_heap = BinaryHeap::new();
    max_heap.push((arr[0], 0));
    for _ in 0..k
    {
        let item = max_heap.pop().unwrap();
        res.push(item.0);
        let left_idx = 2 * item.1 + 1;
        if left_idx < arr.len(){
            max_heap.push((arr[left_idx], left_idx))
        }
        let right_idx = 2 * item.1 + 2;
        if right_idx < arr.len(){
            max_heap.push((arr[right_idx], right_idx))
        }
    }
    res

}

#[test]
fn test_maxk(){
    let max_heap = vec![561,314,401,28,156,359,271,11,3];
    let r = max_k(max_heap, 4);
    for i in [561, 314,401,359]{
        assert!(r.contains(&i));
    }
}