



fn can_reach_end(arr: Vec<usize>) -> bool {

    let mut furthest_reach_so_far = 0;
    let last_idx = arr.len() -1 ;

    let mut i = 0;
    while i <= furthest_reach_so_far && furthest_reach_so_far < last_idx {

        furthest_reach_so_far = furthest_reach_so_far.max(arr[i] + i);
        i += 1;
    }

    furthest_reach_so_far >= last_idx
}


//  using greedy 
fn can_jump(arr: Vec<usize>) -> bool {

    let mut goal = arr.len() - 1;


    for i in (0..arr.len()).rev(){

        if arr[i] + i >= goal {
            goal = i;
        }
    }
    goal == 0

}



#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_can_jump(){
        let arr = vec![3,3,1,0,2,0,1];
        assert!(can_jump(arr));
    }
}