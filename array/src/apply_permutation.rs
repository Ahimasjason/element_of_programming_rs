/*
    Array = [5, 6, 7, 8]

    Permutation = [3, 2, 1, 0]


PYTHON TUTOR:

https://pythontutor.com/visualize.html#mode=edit

def permutation(perm, arr):

    for i in range(len(arr)):
        n = i
        while perm[n] > 0:
            perm_n = perm[n]
            arr_perm_n = arr[perm[n]]
            arr[i], arr[perm[n]] = arr[perm[n]] , arr[i]

            tmp = perm[n]
            perm[n] -= len(perm)
            n = tmp

    perm[:] = [a + len(p) for p in perm]

A = [5, 6, 7, 8]
P = [3, 2, 1, 0]

permutation(P,A)





def permutation(perm, A):


    def cyclic_permutation(start, perm, A):
        i, tmp = start, A[start]

        while True:
            next_i = perm[i]
            next_tmp = A[next_i]
            A[next_i] = tmp
            i, tmp = next_i, next_tmp

            if i == start:
                break



    for i in range(len(A)):

        j = perm[i]
        while j != i:
            # look for i and j met in an array
            if j < i:
                break
            j = perm[j]
        else:
            # if i and j is equal do the cyclic shift
            cyclic_permutation(i, perm, A)

    # perm[:] = [a + len(p) for p in perm]

A = [5, 6, 7, 8]
P = [3, 2, 1, 0]

permutation(P,A)

print(P)



*/

pub fn apply_permutation(arr: &mut Vec<isize>, mut perm: Vec<isize>) {
    for i in 0..arr.len() {
        let mut next = i;

        while perm[next] > 0 {
            arr.swap(i, perm[next] as usize);
            let tmp = perm[next];
            perm[next] -= perm.len() as isize;
            next = tmp as usize;
        }
    }
}

pub fn apply_permutation_usize(arr: &mut Vec<usize>, mut perm: Vec<usize>) {
    let mut res: Vec<bool> = vec![false; perm.len()];

    for i in 0..arr.len() {
        let mut next = i;

        while !res[next] {
            arr.swap(i, perm[next]);
            res[next] = true;
            next = perm[next];
        }
    }

    // for i in 0..arr.len() {
    //     let mut next = i ;

    //     while perm[next] > 0 {
    //         arr.swap(i, perm[next] as usize);
    //         let tmp = perm[next];
    //         perm[next] -= perm.len() as isize;
    //         next = tmp as usize;
    //     }
    // }
}

pub fn apply_perm_cyclic(arr: &mut Vec<usize>, perm: Vec<usize>) {
    let n = (*arr).len();
    let mut cyclic_fn = |start: usize| {
        /*
           P = [3, 0, 1, 2]
           A = [5, 6, 7, 8]
           start = 0;
           val_move_to_start =  A[start]

        */
        let mut i = start;
        let mut tmp = arr[i];

        loop {
            let next_i = perm[i];
            let next_tmp = arr[next_i];

            // arr.swap(next_i, i);
            arr[next_i] = tmp;
            i = next_i;
            tmp = next_tmp;

            if i == start {
                break;
            }
        }
    };

    for i in 0..n {
        // i should move to j
        let mut j = perm[i];

        while i != j {
            if j < i {
                break;
            }
            j = perm[j];
        }

        if i == j {
            cyclic_fn(i);
        }
    }
}

pub fn next_permutation(perm: &mut Vec<usize>) {
    let mut inverse_point: isize = (perm.len() - 2) as isize;
    while inverse_point >= 0 && perm[inverse_point as usize] > perm[inverse_point as usize + 1] {
        inverse_point -= 1;
    }

    if inverse_point == -1 {
        return;
    }

    let n = perm.len();
    for i in ((inverse_point as usize + 1)..n).rev() {
        if perm[i] > perm[inverse_point as usize] {
            perm.swap(i, inverse_point as usize);
            break;
        }
    }

    &mut perm[(inverse_point as usize + 1)..].sort();
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_apply_permutation() {
        let mut array = vec![5, 6, 7, 8];

        let permutation = vec![3, 2, 1, 0];

        apply_permutation(&mut array, permutation);
        assert_eq!(array, [8, 7, 6, 5]);
        eprintln!("array ==> {:?}", array);

        let mut array = vec![5, 6, 7, 8];

        let permutation = vec![3, 2, 1, 0];

        apply_permutation_usize(&mut array, permutation);

        eprintln!("array usize  ==> {:?}", array);
        // assert!(true);
        assert_eq!(array, [8, 7, 6, 5]);
        let mut array = vec![5, 6, 7, 8];

        let permutation = vec![3, 2, 1, 0];

        apply_perm_cyclic(&mut array, permutation);
        eprintln!("array usize  cyclic ==> {:?}", array);
        assert_eq!(array, [8, 7, 6, 5]);

        let mut perm = vec![6, 2, 1, 5, 4, 3, 0];
        next_permutation(&mut perm);
        assert_eq!(perm, [6, 2, 3, 0, 1, 4, 5]);
    }
}
