

//! Program takes an array A of n numbers and rearrange A's element to get a new Array
//!  `B` having property that B[0] <= B[1] >= B[2] <= B[3] >= B[4] .... 

fn rearrange(arr: &mut Vec<usize>){

    for mut  i in 0..arr.len() {
        if i >= arr.len(){return}
        if i + 2 > arr.len() { i = i-1;}
        if i % 2 == 0{
            //  just sort
            let _ = &mut arr[i..(i+ 2)].sort();
        } else {
            // reverse sort
            let _ = &mut arr[i..(i+ 2)].sort();
            let _ = &mut arr[i..(i+ 2)].reverse();
        }

    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_rearrange(){

        let mut arr = (0..10).collect::<Vec<_>>();

        rearrange(&mut arr);
        eprintln!("{:?}", arr);

    }
}