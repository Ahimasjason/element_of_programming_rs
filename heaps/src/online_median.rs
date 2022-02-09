//! median of collectio divides the collection into two equal parts
//! max_heap for smaller half element
//! min_heap for maximum half element
//! max_heap will consume element from min_heap



use super::*;
use std::cmp::{PartialOrd, Reverse};
use std::collections::BinaryHeap;




fn online_median_data(sequence: Vec<isize>) -> Vec<f32>{

    // let mut min_heap = Heap::new(PartialOrd::lt);
    // let mut max_heap = Heap::new(PartialOrd::gt);
    
    let mut min_heap = BinaryHeap::new();
    let mut max_heap = BinaryHeap::new();
    
    let mut res = vec![];

    for x in sequence{
        // first push an elment to min heap
        min_heap.push(Reverse(x));
        // pop element from min_heap and push that into max-heap
        // max_heap should have half of min elements and min should keep max
        max_heap.push(min_heap.pop().unwrap().0);

        if max_heap.len() > min_heap.len(){
            min_heap.push(Reverse(max_heap.pop().unwrap()));
        }

        match max_heap.len() == min_heap.len() {
            true => { 
                res.push(
                    0.5 * (min_heap.peek().unwrap().0 + max_heap.peek().unwrap() ) as f32
                );
            },
            false => res.push(min_heap.peek().unwrap().0 as f32),
        }

    }
    res
}




#[cfg(test)]
mod tests {

    use super::*;



    #[test]
    fn test_online_median(){
        let v = vec![1,0,3,5,2,0,1];
        let r  = online_median_data(v);
        assert_eq!(r, [1.0, 0.5, 1.0, 2.0, 2.0, 1.5, 1.0]);
    }
}