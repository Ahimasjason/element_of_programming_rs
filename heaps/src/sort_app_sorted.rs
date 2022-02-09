//! Sort approximately sorted arrays
//! takes input as array and each number in that array is at most k away from 
//! its correctly sorted position
//! (3,-1,2,6,4,5,8) is more than 2 away from its final destination
//!  


use super::*;




pub fn sort_approximately_sorted_array<T, S>(sequence : T, k : usize) -> Vec<S>
    where
        T : IntoIterator<Item=S>, 
        S : Ord + Copy,
    {
        let mut res = vec![];
        let mut heap = Heap::new(std::cmp::PartialOrd::lt);
        let mut iter = sequence.into_iter();
        for _  in 0..k{
            // look out None case
            let item = iter.next().unwrap();
            heap.insert(item);
        }
        for i in iter{
            let e = heap.top().unwrap();
            res.push(e);
            heap.insert(i);
        }
        while heap.len() > 0 {
            res.push(heap.top().unwrap());
        }
        res
    }




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_approx_sort(){
        let mut arr = [3,-1,2,6,4,5,8];
        let res =   sort_approximately_sorted_array(arr, 2);
        arr.sort();
        for i in 0..arr.len() {
            assert_eq!(arr[i], res[i]);
        }
    }
}