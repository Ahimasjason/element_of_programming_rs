


fn square_root(x: usize) -> isize {

    let (mut l, mut h) = (0, x);

    while l <= h {

        let mid = (l + h) / 2;

        let sq = mid * mid;
        if sq <= x {
            // increase
            l = mid +1;
        } else {
            // decrease
            h = mid - 1;
        }
    }
    l as isize - 1
}




#[cfg(test)]

mod tests{

    use super::*;


    #[test]
    fn test_sq_root(){

        assert_eq!(square_root(21), 4);       
    }
}