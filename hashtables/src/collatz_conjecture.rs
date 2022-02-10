use std::collections::HashSet;




fn collatz_conjecture(n: u32) -> bool {

    let mut verified_nums = HashSet::new();
    

    for i in 3..(n+1){

        let mut test_i = i as usize ;

        // if test i 
        let mut sequence = HashSet::new();
    

        while test_i >= i  as usize{

            if sequence.contains(&test_i){
                // it is going in cycle
                return false
            }
            sequence.insert(test_i);
            if test_i % 2 == 1 {
                // test_i is odd

                if verified_nums.contains(&test_i) {
                    break;
                }
                verified_nums.insert(test_i);
                test_i = (3 * test_i + 1) as usize;
            } else {
                // test_i is even
                test_i /= 2;
            }
        }
    }


    true
}





#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_collatz_conjecture(){

        assert!(collatz_conjecture(11));
    }
}