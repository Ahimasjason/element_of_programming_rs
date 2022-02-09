
use std::{hash::Hash, collections::HashMap};

const DICTIONARY : &[&str]= &["debitcard", 
                               "elvis",
                               "silent",
                               "badcredit",
                               "lives",
                               "freedom",
                               "listen",
                               "levis",
                               "money"
                               ];




pub fn find_anagram<T>(interable : impl IntoIterator<Item=T>) -> Vec<Vec<String>>

    where 
        T: Hash + Eq + ToString
    
{
    let mut dict: HashMap<String, Vec<String>> = HashMap::new();
    

    for it in interable {
        let val = (it).to_string();
        let mut s = it.to_string().chars().collect::<Vec<char>>();
        s.sort_by(|a,b| a.cmp(b));
        dict.entry(s.into_iter().collect()).or_default().push(val);
    }

    dict.values()
        .into_iter()
        .filter(|n| n.len() >= 2)
        // .flatten()
        .cloned()
        .collect()

    
}



#[cfg(test)]
mod tests
{

    use super::*;


    #[test]
    fn test_anagram(){

        let r = find_anagram(DICTIONARY);

        let precom :Vec<Vec<String>> = vec![vec!["elvis".to_string(), "lives".to_string(), "levis".to_string()], vec!["silent".to_string(), "listen".to_string()], vec!["debitcard".to_string(), "badcredit".to_string()]];
        assert!(r.iter().all(|n|precom.contains(n)));

    }
}