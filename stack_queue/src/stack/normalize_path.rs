




fn simplified_path(s: String) -> String {

    let mut stack = Vec::new();

    let mut curr =String::new();

    for c in s.chars(){
        match  c {
            '/' => {
                if curr == "..".to_owned(){
                   if !stack.is_empty(){
                       stack.pop();
                   }
                }
                if curr != "." && curr != "" { stack.push(curr);}
                curr = String::new();
            },
            _ => {
                curr.push(c);
            },
        }
        


    }
    let mut res = "/".to_owned();

    for s in stack {
        res.push_str(&s);
        res.push('/');
    }
    res
}




#[test]
fn test_well_formed(){

    let s = "sc//./../tc/awk/././";
    let res = simplified_path(s.to_string());
    assert_eq!(res, "/../tc/awk/");
    // eprintln!("{:?}", res);
}