pub fn spiral_order1(arr: Vec<Vec<usize>>) -> Vec<usize> {
    /*

        vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![6,7,8],
        ]

    vec![
            1, 2, 3,
            4, 5, 6,
            6, 7, 8,
        ]


    */
    let mut spiral_order = Vec::<usize>::new();

    let len = arr.len();
    let mut clock_wise = |offset: usize| {
        //  len = 2
        //  offset = 0
        // 2-0-1 = 1
        //  offset = 1
        //  2 - 1 - 1
        if offset == len - offset - 1 {}
    };
    for offset in 0..((arr.len() + 1) / 2) {

        // for top in offset
    }

    todo!()
}

pub fn spiral_order2(mut arr: Vec<Vec<usize>>) -> Vec<usize> {
    let SHIFT: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut direction = 0;
    let mut x = 0;
    let mut y = 0;

    // we are going to visit all the elements inside matrix so use n * n
    let mut spiral_order_mat = Vec::<usize>::with_capacity(arr.len() * arr.len());

    let r = std::ops::Range::<isize> {
        start: 0,
        end: arr.len() as isize,
    };

    for i in 0..(arr.len() * arr.len()) {
        spiral_order_mat.push(arr[x][y]);
        // mark the space as visited
        arr[x][y] = 0;

        let (a, b) = SHIFT[direction as usize];
        let mut next_x: isize = x as isize;
        let mut next_y: isize = y as isize;

        match (a, b) {
            (0, 1) => next_y += 1,
            (1, 0) => next_x += 1,
            (0, -1) => next_y += -1,
            (-1, 0) => next_x += -1,

            _ => unimplemented!(),
        }

        if !r.contains(&next_x) || !r.contains(&next_y) || arr[next_x as usize][next_y as usize] == 0
        {
            direction = (direction + 1) & 3;

            let (a, b) = SHIFT[direction];
            next_x = x as isize + a;
            next_y = y as isize + b;
        }

        x = next_x as usize;
        y = next_y as usize;
    }

    spiral_order_mat
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_spiral() {
        let arr = vec![vec![1, 2, 3], vec![4, 5, 6], vec![6, 7, 8]];

        assert_eq!(spiral_order2(arr), [1, 2, 3, 6, 8, 7, 6, 4, 5]);
    }
}
