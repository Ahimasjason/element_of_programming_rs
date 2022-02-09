//! 
//!  Array `[57,131,493,294,221,339,418,452,442,190]`
//!          0   1   2   3  4    5   6   7   8   9
//! 
//! Array will have two State
//! 1) Increasing
//!     When [i-1] < i
//! 2) Decreasing 
//!     When [i-1] >= i
//! 
//!
//! 

use std::cmp::PartialOrd;
use super::*;

use self::State::*;

#[allow(dead_code)]

#[derive(PartialEq)]
enum State{
    Increasing,
    Decreasing,
}


// fn cond(){

// }


// fn merge_sorted_arrays(arrays : Vec<Vec<isize>>) -> Vec<isize> {


//     let mut heap = Heap::new(PartialOrd::lt);

//     let mut iter = arrays.iter().map(|i| i.iter()).collect::<Vec<_>>();

//     let mut res = vec![];
//     for (idx, it) in iter.iter_mut().enumerate(){

//         let elem = it.next();
//         if elem.is_some() {

//             heap.insert((elem.unwrap(), idx));
//         }
//     }

//     while heap.len() > 0 {

//         let smaller_elem = heap.top().unwrap();
//         let next_iter = iter[smaller_elem.1].next();

//         if next_iter.is_some(){
//             heap.insert(())
//         }

//     }
    
//     res
// } 

pub fn sort_k_increasing_decreasing_arr(arr: Vec<usize>) -> Vec<usize> {


    let mut res = Vec::new();

    let mut state = Increasing;
    let mut start_idx : usize = 0;

    for i in 1..arr.len() + 1 {

        if (i == arr.len()) ||
            // If array is in increasing order but state is decreasing    
           ((arr[i-1]< arr[i]) && state == Decreasing) ||
            // array is in decreasing order but state is increasing     
           ((arr[i-1] >= arr[i]) && state == Increasing) 
           {
               
               match state {
                   Increasing => {
                        //    just copy until start index to i
                        let a = &arr[start_idx..i];
                        res.push(a.to_vec());
                        start_idx = i;
                        state = Decreasing;
                        

                   },
                   Decreasing => {
                       let mut a = arr[start_idx..i].to_vec();
                        let len = a.len() - 1;
                       for j in 0..(a.len() /2){
                            a.swap(j, len - j );
                       }
                       res.push(a);
                       start_idx = i;
                       state = Increasing;

                   },
               }

           }  

    }

    crate::merge_sorted_array::merge_sorted_arrays(res)
}



use std::collections::HashMap;

fn grouper<T: Ord + std::hash::Hash >(arr : Vec<(T,T)>) -> HashMap<T,Vec<T>> {



    arr.into_iter().fold(HashMap::new(), |mut acc, a| {
        acc.entry(a.0).or_default().push(a.1);
        acc
    })

} 


#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_sork_mountain_array(){


        let a = vec![57,131,493,294,221,339,418,452,442,190];
        let res = sort_k_increasing_decreasing_arr(a);

        assert_eq!(
            res,
            [57,131,190,221,294,339,418,442,452,493]
        );
    }


    #[test]
    fn test_grouper(){

        let data = vec![(1, 2), (2, 3), (1, 1), (2, 4), (3, 5)];
        let grouped = vec![(1, vec![2, 1]), (2, vec![3, 4]), (3, vec![5])];
        assert_eq!(grouper(data).into_iter().collect::<Vec<_>>(), grouped);
    }
}




