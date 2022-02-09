

use std::collections::{VecDeque, HashMap};

pub fn split_sentence<'a> (dict: &Vec<&'a str>, sentence: &str) -> Option<Vec<&'a str>>

{
    if sentence.len() == 0{ return Some(vec![])}
    for word in dict{
        
        if sentence.starts_with(word){

            let suffix = &sentence[word.len()..];
            let split = split_sentence(dict, suffix);
            if split.is_some(){
                let mut v = vec![];
                v.push(*word);
                // v.copy_from_slice(split.unwrap().as_slice());
                return Some([v, split.unwrap()].concat())
            }
        }
    }
    None
}



pub fn split_sentence_bfs<'a> (dict: &Vec<String>, sentence: String) -> Vec<String> 

{

    let mut map = HashMap::new();
    map.insert("".to_string(), vec![]);

    let mut prefixes = VecDeque::new();
    prefixes.push_back("".to_owned());
    while !prefixes.is_empty(){
        let prefix = prefixes.pop_front().unwrap();
        if prefix == sentence {
            return map[&prefix].to_vec()
        }
        for word in dict {
            if sentence[prefix.len()..].starts_with(word){
                let next_prefix = format!("{}{}", &prefix, &word);
                if !map.contains_key(&next_prefix){
                    let p = &map[&prefix];
                    let mut new = vec![];
                    new.push(word.to_owned());
                    for i in p{
                        new.push(i.to_owned());
                    }
                    let a = (&next_prefix).to_owned();

                    map.insert(next_prefix, new);
                    prefixes.push_back(a);
                }
            }
        }
    }
    vec![]
}







#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_sentence(){ 

        let d  = vec!["ice","cream" , "icecream"];

        let res = split_sentence(&d, &"icecreamicecream"[..]);
        eprintln!("{:?}", res);
    }

    #[test]
    fn test_split_sentence_bfs(){ 

        let d  = vec!["ice".to_owned(),"cream".to_owned() , "icecream".to_owned()];

        let res = split_sentence_bfs(&d, "icecreamicecream".to_owned());
        eprintln!("{:?}", res);
    }
}