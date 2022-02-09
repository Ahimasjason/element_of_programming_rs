use super::count_bits::count_bits;

// Parity of binary word is 1 if the number of 1s in the word is odd
pub fn parity_check(mut n: usize) -> u8 {
    let mut res: u8 = 0;

    while n > 0 {
        // extract the last bit from the n and xor with res
        res ^= (n & 1) as u8;
        n >>= 1;
    }
    res
}

pub fn parity_with_count(n: isize) -> u8 {
    if count_bits(n as isize) % 2 == 0 {
        0
    } else {
        1
    }
}

pub fn parity_by_clearbit(mut n: usize) -> u8 {

    let mut res = 0;
    while n > 0 {
        res ^= 1;
        n &= n - 1;

    }
    res
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parity_count() {

        assert_eq!(1,parity_with_count(1));
        assert_eq!(0,parity_with_count(15));
        assert_eq!(0,parity_with_count(-15));
    }

    #[test]
    fn test_parity() {

        assert_eq!(1,parity_check(1));
        assert_eq!(0,parity_check(15));
    }

    #[test]
    fn test_clearbit_parity() {

        assert_eq!(1,parity_by_clearbit(1));
        assert_eq!(0,parity_by_clearbit(15));
    }


    
}
