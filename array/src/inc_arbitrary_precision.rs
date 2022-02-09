



pub fn plus_one(arr : &mut Vec<isize>) {

    assert!(arr.len() > 0);

    let len = arr.len();

    arr[len - 1] += 1;

    for i in (1..len).rev(){
        if arr[i] != 10 {
            break;
        }

        arr[i-1] += 1;
        arr[i] = 0;
    }

    if arr[0] == 10 {
        arr[0] = 1;
        arr.push(0);
    }
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_plus_one(){

        let mut arr = vec![1,2,9];
        plus_one(&mut arr);
        assert_eq!(arr, [1,3,0]);

        let mut arr = vec![9,9];
        plus_one(&mut arr);
        assert_eq!(arr, [1,0,0]);
    }
}