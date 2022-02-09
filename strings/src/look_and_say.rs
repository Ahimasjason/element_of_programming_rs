



fn next_number(s: String) -> String {

    let mut result = Vec::new();
    let mut i = 0;
    let mut chars = s.chars();

    while i < s.len() {
        let mut count  = 1;
        while (1 + i) < s.len() && chars.nth(i) == chars.nth(i + 1){
            count +=1 ;
            i += 1
        }
        let string_at_index =  chars.nth(i).unwrap();
        result.push(count.to_string() + &string_at_index.to_string());
        i += 1;
    }
    result.into_iter().collect()
}





#[test]
fn test_next_number(){

    let s = next_number(String::from("1"));
    assert_eq!(s, "11");
}