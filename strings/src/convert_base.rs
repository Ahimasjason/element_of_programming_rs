use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref HEX_DIGITS: std::collections::HashMap<char, usize> = {
        let arr = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A',
            'B', 'C', 'D', 'E', 'F',
        ];
        let mut hash_map = std::collections::HashMap::new();

        for (i, c) in arr.into_iter().enumerate() {
            hash_map.insert(c, i);
        }
        hash_map
    };
    static ref ARRAY: [char; 22] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B',
        'C', 'D', 'E', 'F',
    ];
}

fn construct_from_base(num: usize, base: usize) -> String {
    match num {
        0 => String::new(),
        _ => {
            construct_from_base(num / base, base)
                + &ARRAY[num % base].to_ascii_uppercase().to_string()
        }
    }
}

fn base_convert(string: String, b1: usize, b2: usize) -> String {
    let mut reduce_result = 0;
    let mut is_neg = false;
    for (i, c) in string.chars().enumerate() {
        match i {
            0 if c == '-' => {
                is_neg = true;
                continue;
            }
            _ => match HEX_DIGITS.get(&c.to_ascii_lowercase()) {
                Some(index) => {
                    reduce_result = (reduce_result * b1) + (*index);
                }
                None => continue,
            },
        }
    }
    if is_neg {
        String::from("-") + &construct_from_base(reduce_result, b2)
    } else {
        construct_from_base(reduce_result, b2)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_base_convert() {
        eprintln!(
            "base convert 615 {:?} ",
            base_convert(615.to_string(), 7, 13)
        );

        assert_eq!(base_convert(615.to_string(), 7, 13), "1A7");
        assert_eq!(base_convert(String::from("-615"), 7, 13), "-1A7");
    }
}
