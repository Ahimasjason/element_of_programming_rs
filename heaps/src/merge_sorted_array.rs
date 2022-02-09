

use std::cmp;

use super::*;


#[derive(Debug, Copy,Clone,Eq)]
struct Order<T : Ord + Copy >{
    val : T,
    index : usize,
}


impl<T :Ord + Copy>  PartialEq for  Order<T> {

    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl <T: Copy + Ord> Ord for Order<T> {

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl <T: Ord + Copy> PartialOrd for  Order<T> { 

    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.val.cmp(&other.val))
    }
}


pub fn merge_sorted_arrays(arrays : Vec<Vec<usize>>) -> Vec<usize>{
    
    let mut array_iters = arrays.iter().map(|i| i.iter()).collect::<Vec<_>>();

    let mut heap = Heap::<Order<usize>>::new(cmp::PartialOrd::le);
    
    for (i, it) in array_iters.iter_mut().enumerate() {
        let elem = it.next();
        if elem.is_some(){
            heap.insert(Order{val: *elem.unwrap(),index: i });
        }
    }

    let mut res = vec![];

    while heap.len() > 0 {

        let min_val = heap.top().unwrap();
        
        let smallest_arr_list = &mut array_iters[min_val.index];
        res.push(min_val.val);

        let next_elem = smallest_arr_list.next();
        if next_elem.is_some() {
            heap.insert( Order {val : *next_elem.unwrap(), index: min_val.index});
        }

    }
    res
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_order(){

        assert!(Order{val: 1, index: 10} < Order{val: 4, index: 9});
        assert!(Order{val: 1, index: 10} == Order{val: 1, index: 9});
        assert!(Order{val: 2, index: 10} >= Order{val: 1, index: 20});
    }

    #[test]
    fn test_merge_sorted_arrays(){

        let arr = vec![vec![3,5,7],vec![0,6],vec![0,6,8]];
        let r = merge_sorted_arrays(arr);
        assert_eq!(r,[0,0,3,5,6,6,7,8])

    }
}