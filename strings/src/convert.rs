pub fn int_to_string(mut num: isize) -> String {
    let mut is_neg = false;
    if num < 0 {
        is_neg = true;
    }
    

    let mut res = vec![];
    let zero = '0' as u8;
    loop {
        let n: u8 = (num % 10).abs() as u8 + zero;
        let c: String = (n as char).to_string();
        res.push(c);
        num /= 10;
        if num == 0 {
            break;
        }
    }
    let mut r: String;
    if is_neg {
        r = String::from("-");
    } else {
        r = String::new();
    }
    for i in res.into_iter().rev() {
        r.push_str(&i);
    }

    r
}

pub fn string_to_int(string: String) -> isize {
    let mut result: isize = 0;
    let mut is_neg = false;
    for (i, c) in string.chars().enumerate() {
        if i == 0 {
            match c {
                '-' => {
                    is_neg = true;
                }
                _ => {}
            }
        }

        match c.to_digit(10) {
            Some(n) => result = (10 * result) + n as isize,
            None => {}
        }
    }
    if is_neg {
        result = -result;
    }
    result.try_into().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_int_to_string() {
        assert!(int_to_string(314) == "314".to_string());
        assert!(int_to_string(-314) == "-314".to_string());
        assert_eq!(314, string_to_int("314".to_string()));
        assert_eq!(-314, string_to_int("-314".to_string()));
    }
}
