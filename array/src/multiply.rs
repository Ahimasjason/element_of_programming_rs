use std::ops::{Add, AddAssign, Mul, Rem};







fn multiply_arr(num1: &mut Vec<isize>, num2 : &mut Vec<isize>) -> Vec<isize>
{

    let sign = {

        if num1[0] < 0 || num2[0]< 0 {
            -1
        } else {
            1
        }
    };

    if let Some(first) = num1.first_mut()  {
        *first = first.abs();
    }
    if let Some(first) = num2.first_mut()  {
        *first = first.abs();
    }

    let mut res = vec![0; num1.len() + num2.len()];
    for i in (0..num1.len()).rev(){
        for j in (0..num2.len()).rev(){
            res[i+j+1] += num1[i] * num2[j];
            res[i + j] += res[i+ j+1] / 10;
            res[i+j+1] %= 10;
        }
    }
    let idx = res.iter().position(|&i| i != 0);
    if idx.is_some(){
        res = res[idx.unwrap()..].to_vec();
    }
    if let Some(first) = res.first_mut() {
        *first *= sign;
    }
    return res
}



#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_multiply(){
        let mut a = [1,9,3,7,0,7,7,2,1].to_vec();
        let mut b = [-7,6,1,8,3,8,2,5,7,2,8,7].to_vec();

        let res = multiply_arr(&mut a,&mut  b);
        eprintln!("{:?}", res);

        eprintln!("{:?}", multiply_arr(&mut [2,3].to_vec(), &mut [4,2].to_vec()));
    }
}