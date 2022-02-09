

use rand::{self, Rng};



fn find_k_th_largest<T: Ord + Copy>(arr: &mut Vec<T>, k: usize) -> Option<T>

{

    let mut rng = rand::thread_rng();
    let mut left = 0;
    let mut right = arr.len() - 1;

    let mut partition_around = |left:usize , right : usize, pivot_idx : usize|{

        let mut pivot_val = arr[pivot_idx];
        // move the pivot value to right idx
        arr.swap(right, pivot_idx);
        //  swap higher value to left
        let mut new_pivot_idx = left;
    
        for i in left..right{

            // if ith place is higher then move it to left
            if arr[i] > pivot_val {

                arr.swap(i, new_pivot_idx);
                new_pivot_idx += 1;
            }
        }

        // now bring back right to the pivot psosition
        arr.swap(right, new_pivot_idx);
        new_pivot_idx
    };



   

    while left <= right {

        let pivot_idx : usize = rng.gen_range(left..=right);
        let new_pivot_idx = partition_around(left, right, pivot_idx);

        if new_pivot_idx == k - 1 {
            return Some(arr[new_pivot_idx])
        }else if new_pivot_idx > k - 1{
            right = new_pivot_idx - 1;
        } else {
            left = new_pivot_idx + 1;
        }

    }

    None
}


// place the number in value order and find the median
// if odd number then 3 / 2 

fn find_median(arr: &mut Vec<isize>) -> Option<f64> {

    let mut left  = 0;
    let mut right = arr.len() - 1;
    let is_even = arr.len() % 2 == 0;
    let half = arr.len() / 2;

    let mut rng = rand::thread_rng();


    let mut partition = |l: usize, r: usize, p : usize | {

        let pivot_val = arr[p];
        let mut new_pivot_idx = l;
        eprintln!(" Before {:?}, {:?}", arr, new_pivot_idx);
        arr.swap(p, r);

        for i in l..r {
            if arr[i] < pivot_val {
                arr.swap(new_pivot_idx, i);
                new_pivot_idx += 1;
            }
        }
        arr.swap(r, new_pivot_idx);
        eprintln!("{:?}, {:?}", arr, new_pivot_idx);
        new_pivot_idx
    };



    while left <= right {

        let p_index =  rng.gen_range(left..=right);
        let new_pivot_idx = partition(left, right, p_index);
        eprintln!("new p Index {:?}", new_pivot_idx);
        if new_pivot_idx == half { 

            if is_even {
                if new_pivot_idx - 1 > 0 {
                    return Some(0.5f64 * (arr[new_pivot_idx - 1] + arr[new_pivot_idx]) as f64);
                }
            }
            return Some(arr[new_pivot_idx] as f64);
        }


        if new_pivot_idx < half {
            left = new_pivot_idx + 1;
        } else {
            right = new_pivot_idx - 1;
        }
        
        // if is_even {
            
        // }
    }

    None

}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_k_th_largest(){

        let mut arr = vec![3,2,1,5,4];
        assert_eq!(find_k_th_largest(&mut arr, 2),Some(4) );   
    }


    #[test]
    fn test_find_median(){

        let mut arr = vec![3,2,1,5,4];
        let median = find_median(&mut arr);
        assert_eq!(median, Some(3f64));
        let mut arr = vec![3,2,1,5,4, 6];
        let median = find_median(&mut arr);
        assert_eq!(median, Some(0.5 * (3f64 + 4f64)));

        let mut arr = vec![13, 23, 11, 16, 15, 10, 26];
        let median = find_median(&mut arr);
        assert_eq!(median, Some(15f64));
    }
}