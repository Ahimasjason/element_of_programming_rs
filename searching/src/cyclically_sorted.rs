



pub fn smallest_elem(arr: Vec<isize>) -> isize {
    let mut h = arr.len() - 1;
    let mut l = 0;
    while l < h {
        let mid = (l + h) / 2;
        if arr[mid] > arr[h]{
            // mid is higher than its last index so the min element should be at mid + 1 .. psosition
            l = mid +1;
        } else {
            // now mid is less but we do not know whether mid is less or not so include the mid index as well for future computation
            h = mid;
        }
    }
    l as isize
}


/*
     12   |   14  |  18  |  21  |  3  |  6  |  8 |  9
*/ 

pub fn search_sorted(arr: Vec<isize>, k : isize) -> isize {

    let (mut l, mut h) = (0, arr.len() - 1);

    while l <= h {

        let mid = (l + h) / 2;

        if arr[mid] == k { return mid as isize}


        if arr[mid] <= arr[h] {
            // right part is sorted

            if k > arr[mid] && k <= arr[h]{
                l = mid +1;
            } else {
                h = mid - 1;
            }
        
        } else {
            //  if right half is not sorted the the left half is defifitly is sorted
            // arr[low] <= arr[mid]

            if arr[l] <= k &&  k < arr[mid] {
                h = mid - 1;
            } else {
                l = mid + 1;
            }
        }


    }

    -1
}



#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_cyclically_sorted(){

        let arr = vec![378,478,550,631,103,203,220,234,279,368];
        let idx = search_sorted(arr, 478);
        eprintln!("{:?}", idx);
    }


    #[test]
    fn test_search_cyclically_sorted(){

        let arr = vec![378,478,550,631,103,203,220,234,279,368];
        let idx = smallest_elem(arr);
        eprintln!("{:?}", idx);
    }
}