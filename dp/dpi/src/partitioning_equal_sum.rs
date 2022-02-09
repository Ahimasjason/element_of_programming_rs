//! number [2,3,5,6] can be partitioned into [2,6] and [3,5]

use std::collections::HashMap;


fn _can_partition(arr: &Vec<isize>, index: usize, target_sum: isize, cache:&mut HashMap<(usize,isize), bool>) -> bool {

    if target_sum == 0{
        return true
    }

    if target_sum < 0 || index >= arr.len() { return false}


    if cache.contains_key(&(index,target_sum)){
        cache[&(index,target_sum)]
    } else {
        let t_sum = target_sum - arr[index];
        let including = _can_partition(arr, index + 1, t_sum, cache);
        cache.insert((index,t_sum), including);
        let without_including = _can_partition(arr, index + 1, target_sum, cache);
        cache.insert((index,target_sum), without_including);
        including || without_including
    }
}


fn can_partition(arr: Vec<isize>) -> bool {
   
   let total_sum: isize = arr.iter().sum();
   if total_sum % 2 == 1 {
       return false
   }
   _can_partition(&arr, 0, total_sum / 2 , &mut HashMap::new())
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_can_partition(){ 

        let arr = vec![2,3,5,6];

        assert!(can_partition(arr));

        let arr = vec![2,3,5,7];

        assert!(!can_partition(arr));
    }
}