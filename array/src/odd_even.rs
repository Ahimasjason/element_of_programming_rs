fn odd_even(arr: &mut [isize]) {
    let (mut next_even, mut next_odd) = (0, arr.len() - 1);

    while next_even <= next_odd {
        if arr[next_even] % 2 == 0 {
            next_even += 1
        } else {
            arr.swap(next_even, next_odd);
            next_odd -= 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_odd_even() {
        let mut a = [1, 2, 3, 4, 5, 6, 7];
        odd_even(&mut a);
        eprintln!("{:?}", a);
    }
}
