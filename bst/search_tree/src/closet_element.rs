use super::*;
use std::collections::BTreeMap;

fn closest_elem_in_sorted_array(mut array: Vec<Vec<isize>>) -> isize {
    let mut iters = BTreeMap::new();

    for (idx, arr) in array.iter().enumerate() {
        let mut iter = arr.iter();
        if let Some(num) = iter.next() {
            iters.insert((num, idx), iter);
        }
    }

    loop {}
    unimplemented!()
}

fn same_element(arr1: Vec<isize>, arr2: Vec<isize>, arr3: Vec<isize>) -> Vec<isize> {
    let (mut a, mut b, mut c) = (0, 0, 0);

    let mut res = vec![];

    while (a < arr1.len() && b < arr2.len()) && c < arr3.len() {
        println!("a {} , b {}, c {}  ", a, b, c);

        if (arr1[a] == arr2[b]) && (arr2[b] == arr3[c]) {
            if !res.contains(&arr1[a]) {
                res.push(arr1[a]);
                a += 1;
                b += 1;
                c += 1;
            }
        } else if arr1[a] < arr2[b] {
            a += 1;
        } else if arr2[b] < arr3[c] {
            b += 1;
        } else {
            c += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_same_element() {
        let a = vec![5, 10, 15];
        let b = vec![3, 6, 9, 12, 15];
        let c = vec![8, 14, 15];

        assert_eq!(same_element(a, b, c), [15]);
    }
}
