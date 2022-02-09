
fn d2_search(arr : Vec<[isize; 5]>, x: isize) -> bool {

    let mut row = 0;
    let mut col = arr[0].len() - 1;

    //  row will be increasing and col is decreasing
    while row < arr.len(){

        if arr[row][col] == x {
            return true
        }

        if arr[row][col] < x {
            row += 1;
        } else {
            //  column is greater and now decrease column
            if col == 0 { break }
            col -= 1;
        }
    }

    false
}





#[cfg(test)]
mod tests {

    use super::*;

    const ARRAY : [[isize;5];6] = [
                                    [-1,2,4,4,6],
                                    [1,5,5,9,21],
                                    [3,6,6,9,22],
                                    [3,6,8,10,24],
                                    [6,8,9,12,25],
                                    [8,10,12,13,40],
                                ];


    #[test]
    fn test_2d_search(){
        assert!(d2_search(ARRAY.to_vec(), 8));
        assert!(!d2_search(ARRAY.to_vec(), 7));
        
    }


}