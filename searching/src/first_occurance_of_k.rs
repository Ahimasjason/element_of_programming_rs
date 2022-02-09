



pub(super) fn search_first_of_k(arr: Vec<isize>, key: isize) -> isize {

    let mut l = 0;
    let mut h = arr.len();
    let mut res = -1;

    while l <h {
        let mid = (l + h) / 2;
        match arr[mid].cmp(&key){
            std::cmp::Ordering::Less => {
                l = mid +1;
            },
            std::cmp::Ordering::Equal => {
                res = mid as isize;
                h = mid - 1;

            },
            std::cmp::Ordering::Greater => {
                h = mid - 1;
            },
        }
    }

    res
}


fn search_index_fisrt_occ_gt(arr: Vec<isize> , key : isize) -> isize {

    let mut l = 0;
    let mut h = arr.len();
    let mut res = 1;
    
    while l < h {

        let mid = (l + h) / 2;
        match arr[mid].cmp(&key){
            std::cmp::Ordering::Less => {
                l = mid +1;
            },
            std::cmp::Ordering::Equal => {
                if mid + 1 < arr.len() {
                    res = mid as isize + 1;
                }
                l = mid + 1;

            },
            std::cmp::Ordering::Greater => {
                h = mid - 1;
            },
        }
    }
    res
}

fn enclosing_intervals(arr: Vec<isize> , key : isize) -> (isize, isize) {

    let mut r = (-1,-1);

    let mut l = 0;
    let mut h = arr.len() - 1;
    while l <= h {
        let mid = (l + h) / 2;
        match arr[mid].cmp(&key){
            std::cmp::Ordering::Less => {
                l = mid +1;
            },
            std::cmp::Ordering::Equal => {
                r.0 = mid as isize;
                h = mid - 1;
                

            },
            std::cmp::Ordering::Greater => {
                h = mid - 1;
            },
        }
    }
    if r.0 >= 0 {

        l = r.0 as usize;
        h = arr.len() - 1;

        while l <= h {
        let mid = (l + h) / 2;

        match arr[mid].cmp(&key){

            std::cmp::Ordering::Less => {
                l = mid +1;
            },
            std::cmp::Ordering::Equal => {
                r.1 = mid as isize;
                l = mid + 1;
            },
            std::cmp::Ordering::Greater => {
                h = mid - 1;
            },
        }
    }   

    }
    r
}
#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_search_first_of_k(){
        let arr = vec![-14,-10,2,108,108,243,285,285,285,401];
        assert_eq!(search_first_of_k(arr, 108), 3);
    }


    #[test]
    fn test_search_index_fisrt_occ_gt(){
        let arr = vec![-14,-10,2,108,108,243,285,285,285,401];

        assert_eq!(search_index_fisrt_occ_gt(arr, 285), 9);

    }

    #[test]
    fn test_enclosing_intervals(){
        let arr = vec![1,2,2,4,4,4,7,11,11,18];
        let ans = enclosing_intervals(arr, 11);
        assert_eq!(ans, (7,8));
        let arr = vec![1,2,2,4,4,4,7,11,11,18];
        let ans = enclosing_intervals(arr, 13);
        assert_eq!(ans, (-1,-1));

    }
}

