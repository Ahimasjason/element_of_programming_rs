//! Dutch flag partition without any reference

use std::cmp::Ord;

fn dutch_flag<T: Ord + Copy>(pivot_idx: usize, arr: &mut [T]) {
    let pivot = arr[pivot_idx];

    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[j] < pivot {
                arr.swap(i, j);
                break;
            }
        }
    }

    for i in (0..arr.len()).rev() {
        if arr[i] < pivot {
            break;
        }
        for j in (0..i).rev() {
            if arr[j] > pivot {
                arr.swap(i, j);
                break;
            }
        }
    }
}

fn dutch_flag1<T: Ord + Copy>(pivot_idx: usize, arr: &mut [T]) {
    let pivot = arr[pivot_idx];

    let mut smaller = 0;

    for i in 0..arr.len() {
        if arr[i] < pivot {
            arr.swap(i, smaller);
            smaller += 1;
        }
    }

    let mut larger = arr.len() - 1;

    for i in (0..arr.len()).rev() {
        if arr[i] < pivot {
            break;
        }

        if arr[i] > pivot {
            arr.swap(i, larger);
            larger -= 1;
        }
    }
}

pub fn dutch_flag2<T: Ord + Copy>(pivot_idx: usize, arr: &mut [T]) {
    let pivot = arr[pivot_idx];
    let (mut smaller, mut equal, mut larger) = (0, 0, arr.len() - 1);
    while equal < larger {
        if arr[equal] < pivot {
            arr.swap(smaller, equal);
            smaller += 1;
            equal += 1;
        } else if arr[equal] == pivot {
            equal += 1;
        } else {
            arr.swap(equal, larger);
            larger -= 1;
            equal += 1;
        }
    }
}
