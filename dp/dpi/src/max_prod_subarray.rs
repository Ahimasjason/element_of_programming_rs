

// pub fn find_max_prod(arr: &Vec<isize>) -> usize 
// {

//     let mut max_prod_forward = 0;
//     let mut max_prod_rev = 0;

//     let mut prod_fr: usize = 1;
//     let mut prod_rev = 1;

//     let size = arr.len() - 1;
//     for forward_idx in 0..arr.len() {

//         // perform forward analysis

//         let reverse_idx = size - forward_idx;

//         if prod_fr == 0 {
//             prod_fr = arr[forward_idx];
//         } else {
//             prod_fr *= arr[forward_idx];
//         } 
//         max_prod_forward = max_prod_forward.max(prod_fr);

//         //  do reverse analysis

//         if prod_rev == 0 { 
//             prod_rev = arr[prod_rev];
//         } else {
//             prod_rev *= arr[prod_rev]
//         }
        


//     }


//     max_prod_rev.max(max_prod_forward)
// }



fn max_product_subarray(nums : Vec<isize> ) -> isize 
{

    let mut max_chunk = isize::MIN;
    let mut min_chunk = isize::MAX;
    for chunk in nums {
        let max_new_chunk = chunk;
        let max_cnt_pos: isize;
        let max_flip_neg: isize;
        // look for continue
        if max_chunk > 0 && chunk > 0 {
            max_cnt_pos = max_chunk * chunk;
        } else {
            max_cnt_pos = isize::MIN;
        }
        // flip if value and min value is negative

        if min_chunk < 0 && chunk < 0 {
            max_flip_neg = min_chunk * chunk;
        } else {
            //  if i keep isize::MAX the this max will be max for all the iteration
            max_flip_neg = isize::MIN;
        }
        // Do the negative case
       
        let min_new_chunk = chunk;
        let min_cnt_neg: isize;
        let min_flip_pos: isize;
        if min_chunk < 0 && chunk > 0 {
            min_cnt_neg = min_chunk * min_chunk;
        } else {
            min_cnt_neg = isize::MAX;
        }
        // flip the sign

        if max_chunk > 0 && chunk < 0 {
            min_flip_pos = max_chunk * chunk;
        } else {
            min_flip_pos = isize::MAX;
        }
        // eprintln!("new_max -> {}, max-cnt {}, flip -> {} ",max_new_chunk, max_cnt_pos,max_flip_neg);
        max_chunk = max_new_chunk.max(max_cnt_pos).max(max_flip_neg);
        // eprintln!("max-chunk ->  {:?}", max_chunk);
        // eprintln!("new_min -> {}, mix-cnt {}, flip -> {} ",min_new_chunk, min_cnt_neg,min_flip_pos);
        min_chunk = min_new_chunk.min(min_cnt_neg).min(min_flip_pos);
        // eprintln!("min chunk {}", min_chunk);
        // eprintln!();

    }


    if max_chunk == isize::MIN {
        0
    } else {
        max_chunk
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product_subarray(){
        assert_eq!(max_product_subarray(vec![1,2,3]),6);
        assert_eq!(max_product_subarray(vec![1,-2,-3]),6);
        assert_eq!(max_product_subarray(vec![1,-2,-3,-4,-5]),120);
    }
}