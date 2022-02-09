use std::collections::{HashMap, HashSet };
use linked_list::LinkedList;




fn smallest_subarray_covering_set(paragraph : Vec<String> , keywords : HashSet<String>) -> (isize, isize)

{
    let mut result = (-1,-1);
    let mut left = 0;
    let mut keywords_to_cover: HashMap<String, isize> = HashMap::new();
    let mut remaining_to_cover = keywords.len();
    for keyword in keywords{

        *keywords_to_cover.entry(keyword).or_default() += 1;
    }
    for (right, p) in paragraph.iter().cloned().enumerate() {
        if keywords_to_cover.contains_key(&p){
            *keywords_to_cover.get_mut(&p).unwrap() -= 1;
            if *keywords_to_cover.get(&p).unwrap() >= 0 {
                remaining_to_cover -= 1;
            }
        }
        while remaining_to_cover == 0 {
            // move the left pointer
            // all keywords are found now look for smallest subarray
            if result == (-1, -1) || (result.1  - result.0) > (right as isize- left as isize){
                result = (left as isize, right as isize);
            }
            let pl = &paragraph[left];
            if keywords_to_cover.contains_key(pl){
                // now increase the keywords_to_cover because we are moving the left pointer
                *keywords_to_cover.get_mut(pl).unwrap() += 1;
                if *keywords_to_cover.get(pl).unwrap() > 0 {
                    remaining_to_cover += 1;
                }
            }
            left += 1;
        }
    }
    result
}


fn smallest_subarray_covering_set2(paragraph : Vec<String>, keywords: HashSet<String>) -> (isize, isize)
{


    let mut result = (-1, -1);
    let mut lo = LinkedList::new();
    let mut d: HashMap<String, Option<(usize,usize)>> = HashMap::new();

    for k in keywords {
        d.insert(k, None);
    }

    for (right, item) in paragraph.into_iter().enumerate() {
        if d.contains_key(&item){
            println!("right --> {} {}", right, &item);
            // current word exist in keywords
            // look id current word is already none
            println!("{:?}", &d);
            let curr = d.remove(&item).unwrap();

            if curr.is_some(){
                // now remove that word so that new length mabe minimum
                let lo_idx = curr.unwrap();
                eprintln!("before remove  --> {:?} , {:?}", lo, lo_idx);
                lo.remove(lo_idx.1);
            }
            d.insert(item, Some((right, lo.len())));
            eprintln!("before --> {:?}", lo);
            lo.push_back(right);
            eprintln!("after --> {:?}", lo);
            println!("{:?}", &d);


            if lo.len() == d.len() {
            //  found the subarray
            // linke list head is always going to be left
            let left = *lo.front().unwrap();

            if result == (-1,-1) || (right as isize - left as isize) <  result.1 - result.0 {
                result = (left as isize, right as isize);
            }
            }
        }

        
    }
    result
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_subarray_covering_set(){

        let arr = vec![
                "apple",
                "banana",
                "apple",
                "apple",
                "dog",
                "cat",
                "apple",
                "dog",
                "banana",
                "apple",
                "cat",
                "dog"
        ]
        .into_iter()
        .map(ToString::to_string)
        .collect();

        let keywords = HashSet::<String>::from_iter(["banana".into(),"cat".into()]);
    
        assert_eq!((8,10),smallest_subarray_covering_set(arr, keywords));
    }

    #[test]
    fn test_smallest_subarray_covering_set2(){

        let arr = vec![
                "apple",
                "banana",
                "apple",
                "apple",
                "dog",
                "cat",
                "apple",
                "dog",
                "banana",
                "apple",
                "cat",
                "dog"
        ]
        .into_iter()
        .map(ToString::to_string)
        .collect();

        let keywords = HashSet::<String>::from_iter(["banana".into(),"cat".into()]);
    
        assert_eq!((8,10),smallest_subarray_covering_set2(arr, keywords));
    }

}