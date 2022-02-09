macro_rules! swap_values {
    ($a_ref: expr, $b_ref: expr) => {
        let tmp = *$a_ref;
        *$a_ref = *$b_ref;
        *$b_ref = tmp;
    };
}

pub fn rotate(arr: &mut Vec<Vec<usize>>) {
    // Transpose an array

    for i in 0..arr.len() {
        for j in i..arr.len() {
            /*
                            [
                                [1, 2, 3, 4]

                                [5, 6, 7, 8],
                                [9, 10, 11, 12],
                                [13, 14, 15, 16]
                            ]

                            [
                                [1, 5, 9, 13]
                                [2, 6 j, 7, 8],
                                [3, 10, 11, 12],
                                [4, 14, 15, 16]
                            ]
            1            */

            swap_values!(&mut arr[i][j], &mut arr[j][i]);
        }
    }

    for i in 0..arr.len() {
        for j in 0..arr[i].len() / 2 {
            let idx = arr[i].len() - 1 - j;
            swap_values!(&mut arr[i][j], &mut arr[i][idx]);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rotation() {
        let mut arr = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        rotate(&mut arr);
        eprintln!("{:?}", arr);
        assert_eq!(arr[0][3], 1);
    }
}
