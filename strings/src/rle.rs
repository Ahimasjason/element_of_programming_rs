//! Run Length Encoding 
//!  RLE of `aaaabcccaa` is 4a1b3c2a
//!  decoding of `3e4f2e` retruns `eeeffffee`



fn decoding(s: impl AsRef<str>) -> String {
    // '4a1b3c2a'
    let mut res = String::new();
    let mut count = 0;
    let string_vec = s.as_ref().chars();

    for i in string_vec{
        if i.is_digit(10){
            count = (count * 10) + i.to_digit(10).unwrap();
        } else {
            for j in 0..count{
                res.push(i);
            }
            count = 0;
        }

    }
    res
}


fn encoding(s: impl AsRef<str>) -> String {


    let mut res_vec = String::new();
    let string_vec = s.as_ref().chars().collect::<Vec<char>>();
    let mut count = 1;
    for i in 1..(string_vec.len() + 1){

        if i == string_vec.len() || string_vec[i-1] != string_vec[i]{
            
            res_vec.push_str(&format!("{}{}", count, &string_vec[i-1]));
            count = 1;
        } else {
            count +=  1;
        }
    }

    res_vec
   
}

#[test]
fn test_encoding(){
    let s = "aaaabcccaa";
    assert_eq!("4a1b3c2a", encoding(s));
    assert_eq!(s, decoding(encoding(s)));
}


