

use std::collections::BTreeSet;


fn longest_range_subarry(nums : Vec<isize>) -> isize{

    let mut map = nums.iter().cloned().collect::<BTreeSet<isize>>();

    let mut max_res = 0 ;
  

    for x in nums {

        if map.contains(&x){
            let curr = map.take(&x).unwrap();
            let mut lower_bound = curr - 1;
            while map.contains(&lower_bound){
                map.remove(&lower_bound);
                lower_bound -= 1;
            }
            let mut upper_bound = curr + 1;
            while map.contains(&upper_bound){
                map.remove(&upper_bound);
                upper_bound += 1;
            }
            max_res = max_res.max(upper_bound - lower_bound - 1);
        }
    }
   max_res
}



#[cfg(test)]
mod tests {

    use super::*;



    #[test]
    fn test_longest_range_subarry(){
        let ip = vec![3,-2,7,9,8,1,2,0,-1,5,8];
        assert_eq!(longest_range_subarry(ip), 6);
    }
}