use std::collections::HashMap;








pub fn can_form_palindrome(s : &str) -> bool 
{

    let mut map: HashMap<char, usize> = HashMap::new();
    for c in s.chars(){
        *map.entry(c).or_default() += 1;
    }

    map.values()
        .into_iter()
        .fold(0 , |acc, x| acc + x % 2) <= 1
    
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_can_form_palindrome(){
        assert!(can_form_palindrome("edified"));
        
    }
}