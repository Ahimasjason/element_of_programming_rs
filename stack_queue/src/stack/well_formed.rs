
use std::collections::HashMap;


// "[()[]{()()}]"
fn is_well_formed(s : String) -> bool{

    let mut  map = HashMap::new();

    map.insert('[', ']');
    map.insert('{', '}');
    map.insert('(', ')');

    let mut stack = Vec::new();
    for c in s.chars() {

        if map.contains_key(&c){
            stack.push(c);
        } else {
            if stack.is_empty(){
                return false
            }
            let v = stack.pop().unwrap();
            match map.get(&v){
                Some(n) => {
                    if *n != c{
                        return false
                    }
                },
                None => return false
            }



        }
    }

    stack.is_empty()
} 





#[test]
fn test_well_formed(){

    assert!(is_well_formed("[()[]{()()}]".to_string()));
    assert!(!is_well_formed("{)".to_string()));

}