
use std::collections::HashMap;



fn _count_expression_recr(
                        nums: &Vec<isize>, 
                        target_sum:isize, 
                        partial_res:isize,
                        index: usize,
                ) ->isize

{
    if index >= nums.len() {
        if partial_res == target_sum{
            return 1
        } else {
            return 0
        }
    }

    let count_add = _count_expression_recr(nums, target_sum, partial_res + nums[index], index +1);
    let count_sub = _count_expression_recr(nums, target_sum, partial_res - nums[index], index + 1);

    return count_add + count_sub

}

fn count_expression_recr(nums: Vec<isize>,target_sum:isize) -> isize {


    _count_expression_recr(&nums, target_sum, 0, 0)
}


/**
 * 
 * ```python
 * 
 * def f(nums, t):
 *     from collections import Counter
 *     counter = Counter({nums[0]: 1})
 *     index = 1
 *     while index < len(nums):
 *         next_c = Counter()
 *         for (k,v) in counter.items():
 *             next_add = k + nums[index]
 *             next_sub = k - nums[index]
 *             next_c[next_add] += v
 *             next_c[next_sub] += v
 *         counter -next_c
 *         index += 1           
 * ```
 * 
 * 
*/

fn bfs(nums: Vec<isize> , target_sum: isize) -> isize{

    let mut counter = HashMap::new();
    counter.insert(nums[0], 1);
    let mut index = 1;

    while index < nums.len(){

        let mut next_counter = HashMap::new();

        for (k,v) in counter{

            let next_add = k + nums[index];
            let next_sub = k - nums[index];
            *next_counter.entry(next_add).or_insert(0) += v;
            *next_counter.entry(next_sub).or_insert(0) += v; 
        }
        counter = next_counter;
        index += 1;
    }
    println!("{:?}", counter);
    target_sum
}



#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_recr(){
        let v = vec![1,2,3];
        assert_eq!(count_expression_recr(v, 4), 1);
    }


    #[test]
    fn test_recr1(){
        let v = vec![1,1,1,1];
        assert_eq!(count_expression_recr(v, 2), 3);
    }

    #[test]
    fn test_bfs(){
        let v = vec![1,1,1,1];
        assert_eq!(bfs(v, 4), 4);
    
    }
}