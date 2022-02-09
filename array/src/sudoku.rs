use std::collections::HashSet;

pub fn sudoku(arr: Vec<Vec<usize>>) -> bool {
    let has_dups = |block: &Vec<usize>| -> bool {
        let v: Vec<usize> = block.iter().filter(|&c| *c != 0).cloned().collect();
        let h: HashSet<usize> = v.iter().cloned().collect();
        // eprintln!(" result is {}, H => {:?}", h.len() == v.len(), h);
        h.len() != v.len()
    };

    // check for olumn
    for i in 0..arr.len() {
        if has_dups(&arr[i]) || has_dups(&(0..arr.len()).map(|n| arr[n][i]).collect::<Vec<usize>>())
        {
            return false;
        }
    }

    //  LOOK for the diagonal
    /*

    array([[ 0,  1,  2,  3,  4,  5,  6,  7,  8],
       [ 9, 10, 11, 12, 13, 14, 15, 16, 17],
       [18, 19, 20, 21, 22, 23, 24, 25, 26],
       [27, 28, 29, 30, 31, 32, 33, 34, 35],
       [36, 37, 38, 39, 40, 41, 42, 43, 44],
       [45, 46, 47, 48, 49, 50, 51, 52, 53],
       [54, 55, 56, 57, 58, 59, 60, 61, 62],
       [63, 64, 65, 66, 67, 68, 69, 70, 71],
       [72, 73, 74, 75, 76, 77, 78, 79, 80]])

    */

    let region_size = (arr.len() as f64).sqrt() as usize;

    for i in 0..region_size {
        for j in 0..region_size {
            // if has_dups(
            //     &((i * region_size)..(i + 1) * region_size)
            //         .map(|n| (j * region_size)..(j + 1) * region_size)
            //         .map(|k| arr[n][k])
            //         .collect(),
            // ) {}
            let mut region_arr = vec![];

            for a in (i * region_size)..((i + 1) * region_size) {
                for b in (j * region_size)..((j + 1) * region_size) {
                    region_arr.push(arr[a][b]);
                }
            }
            // eprintln!("region_arr --> {:?}", region_arr);
            if has_dups(&region_arr) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sudoku() {
        let sudoku_arr = vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];

        assert_eq!(sudoku(sudoku_arr), true);
    }
}
