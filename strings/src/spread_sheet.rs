fn ss_decode_col_id(col: String) -> usize {
    let a = 'A' as u8;
    let mut result = 0;

    for c in col.chars() {
        let val = (((c as u8) - a) + 1) as usize;
        result = (result * 26) + val;
    }

    result.into()
}

fn id_to_ss_col(mut id: usize) -> String {
    let mut string = Vec::new();

    let a = 'A' as u8;

    loop {
        let v = (id % 26) as u8;
        match v {
            0 => string.push('Z'),
            _ => {
                string.push((a + v - 1) as char);
            }
        }

        id = (id as f64 / 26f64).ceil() as usize;

        id = id - 1;
        if id <= 0 {
            break;
        }
    }
    string.iter().collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ss_decode_col_id() {
        let zz = "ZZ".to_string();
        assert_eq!(702, ss_decode_col_id(zz));

        let a = "A".to_string();
        assert_eq!(1, ss_decode_col_id(a));
    }

    #[test]
    fn test_ss_encode_col_id() {
        // println!("A 702is {:?}", id_to_ss_col(702));
        // println!("A 1 is {:?}", id_to_ss_col(1));
        // println!(
        //     "A 27 is {:?} but {:?}",
        //     id_to_ss_col(27),
        //     ss_decode_col_id("AB".to_string())
        // );
        assert_eq!("A", id_to_ss_col(1));
        assert_eq!("AA", id_to_ss_col(27));
        assert_eq!("ZZ".to_string(), id_to_ss_col(702));

        // let a = "A".to_string();
        // assert_eq!(1, ss_decode_col_id(a));
    }
}
