
//!Design the algorithm that process building east-to-west order 
//! an returns the set of building which view the sunset 



fn east_to_west_order(view : Vec<usize>) -> Vec<usize> {

    let mut stack: Vec<(usize,usize)> = Vec::new();

    for (idx, building_height) in view.into_iter().enumerate(){

        eprintln!("stack -> {:?}", stack);
        while !stack.is_empty() && stack[stack.len() -1 ].1 <= building_height    {
            stack.pop();
        } 
        stack.push((idx, building_height));
    } 

    // stack
    stack.into_iter().rev().map(|v| v.0).collect()
}

fn west_to_east(view: Vec<usize>) -> Vec<usize> {

    let mut stack: Vec<(usize,usize)> = Vec::new();


    for (idx, building_height) in view.into_iter().enumerate(){

      if !stack.is_empty() && stack[stack.len() - 1].1 > building_height {
          continue
      }
      stack.push((idx, building_height));    
    }

    stack.into_iter().map(|v| v.0).collect()
    
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_east_to_west(){

        let arr = vec![4,5,3,4,2];
        assert_eq!(east_to_west_order(arr), [4,3,1]);
    }


    #[test]
    fn test_west_to_east(){
        let arr = vec![2,4,3,5,4];
        assert_eq!(west_to_east(arr), [0,1,3]);
    }
}