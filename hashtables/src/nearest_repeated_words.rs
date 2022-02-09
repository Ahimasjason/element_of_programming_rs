

use std::collections::HashMap;

fn find_nearest_word_repeatition(words : Vec<String>) -> isize 
{

    let mut map: HashMap<&String, usize> = HashMap::new();
    let mut nearest_repeated_distance = isize::MAX;

    for (idx, w) in words.iter().enumerate() {

        if map.contains_key(w){
            let latest_equal_word = *map.get(w).unwrap();
            nearest_repeated_distance = nearest_repeated_distance.min(
                idx as isize - latest_equal_word as isize
            );
        }

        map.insert(w, idx);
    }
    if nearest_repeated_distance == isize::MAX {
        -1
    } else {
        nearest_repeated_distance
    }
}





#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_nearest_rep_seq(){
        let arr = vec![
            String::from("All"),
            String::from("work"),
            String::from("and"),
            String::from("no"),
            String::from("play"),
            String::from("makes"),
            String::from("for"),
            String::from("no"),
            String::from("work"),
            String::from("no"),
            String::from("fun"),
            String::from("and"),
            String::from("no"),
            String::from("results"),
        ];
        assert!(find_nearest_word_repeatition(arr) == 2);
    }
}