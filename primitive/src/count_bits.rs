pub fn count_bits(mut n: isize) -> u8 {
    let mut ans: u8 = 0;

    if n < 0 {
        // use the ngative approach
        loop {
            eprintln!("n is ==> {} , {}", n, n >> 1);
            //  n is 2's complement so the number will stop at - 1
            if n == -1 {
                ans += (n & 1) as u8;
                break;
            }
            ans += (n & 1) as u8;
            n >>= 1;
        }
        ans
    } else {
        while n > 0 {
            ans += (n & 1) as u8;
            n >>= 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pos_count_bits() {
        assert_eq!(3, count_bits(14));
    }

    #[test]
    fn test_neg_count_bits() {
        assert_eq!(1, count_bits(-16));
        assert_eq!(8, count_bits(-18894));
    }
}
