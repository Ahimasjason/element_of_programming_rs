use std::collections::HashMap;





pub fn is_letter_constructible(letter: String, magazine: String) -> bool
{

    let mut map : HashMap<char, usize> = HashMap::new();
    
    for c in letter.chars(){
        *map.entry(c).or_default() += 1;
    }

    for c in magazine.chars(){

        if map.contains_key(&c){

            *map.get_mut(&c).unwrap() -= 1;

            if *map.get(&c).unwrap() == 0 {
                map.remove(&c);
                if map.is_empty(){
                    return true
                }
            }

        }
    }
    map.is_empty()
}



#[cfg(test)]
mod tests {


    use super::*;


    #[test]
    fn test_is_letter_constructible(){

        assert!(
            is_letter_constructible("123".into(),"1123".into())
        );
        assert!(
            !is_letter_constructible("124".into(),"1123".into())
        );
    }
}