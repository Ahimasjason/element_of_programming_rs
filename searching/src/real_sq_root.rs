



pub fn square_root(x: f64)-> f64
{
    let (mut l, mut h) = if x < 1.0 {
        (x, 1.0)
    } else {
        (1.0, x)
    };
    while !((l - h).abs().trunc() <= f64::EPSILON){
        let mid = 0.5 * (l + h);
        let mid_sq = mid * mid;
        if mid_sq > x {
            h = mid;
        } else {
            l = mid;
        }
    } 
    l
}



#[test]
fn test_sq_root(){

    assert_eq!(square_root(21.0),  4.125);
}
