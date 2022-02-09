

fn reverse_words(s: &mut [u8]){
    s.reverse();
    let len = s.len();
    let reverse_range = |arr : &mut [u8]| {

        let arr_len = arr.len();
        for i in 0..arr.len() / 2 {
            arr.swap(i, arr_len - i - 1);
        }
    };

    let mut start = 0 ;
  
    for i in start..len{
        if s[i] == 32 {
            let end = i;
            reverse_range(&mut s[start..end]);
            start = end + 1;

        }
    }
    reverse_range(&mut s[start..len ]);
}



#[test]
fn test_reverse_strings(){

    let mut string = "ram is costly".as_bytes().to_owned();
    
    reverse_words(&mut string);
    assert_eq!(string, "costly is ram".as_bytes());
}