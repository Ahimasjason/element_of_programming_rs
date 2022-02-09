


pub fn search_equal_to_k(arr: Vec<isize>) -> isize {


    let mut l = 0;
    let mut h = arr.len() - 1;

    while l <= h {

        let mid = (l + h) / 2;

        let diff: isize = arr[mid] as isize - mid as isize;
        if diff == 0 {
            return mid as isize
        }
        if diff > 0 {
            h = mid - 1;
        } else {
            l = mid +1;
        }
        
    }

    -1 
}




#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_search_equal_to_k(){
        let arr = vec![-2,0,2,3,6,7,9];

        let r = search_equal_to_k(arr);
        // eprintln!("{:?}", r);
        assert_eq!(3,r);
    }
}