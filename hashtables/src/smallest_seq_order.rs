






use std::collections::HashMap;


fn smallest_sequentially_covering_subset(paragraph : Vec<String> , keywords : Vec<String>) -> (isize, isize)
{
    let keyword_idx : HashMap<String, usize> = keywords
                                                .iter()
                                                .cloned()
                                                .enumerate()
                                                .map(|item| (item.1, item.0))
                                                .collect();

    let mut latest_freq = vec![-1; keywords.len()];


    let mut result = (-1isize, -1isize);
    let mut smallest_subarray = vec![-1; keywords.len()];
    let mut smallest_len = isize::MAX;
    for (idx,x) in (0usize..).zip(paragraph){

        if keyword_idx.contains_key(&x){

                let element_idx = keyword_idx.get(&x).unwrap();

                // If element idx is 0 then update the first index

                if element_idx == &0 {
                    // found the first element
                    smallest_subarray[0] = 1;
                } // if previous element is found then we can include this element
                 else if smallest_subarray[*element_idx - 1] != -1 {
                    //  calculate the distance from this element to the previous element

                    let previous_distance = idx as i32- latest_freq[element_idx -1];
                    smallest_subarray[*element_idx] = previous_distance  + smallest_subarray[element_idx - 1];
                 }

                latest_freq[*element_idx] = idx as i32 ;


                if element_idx == &keywords.len() && smallest_subarray[smallest_subarray.len() - 1]  < smallest_len.try_into().unwrap(){

                    smallest_len = smallest_subarray[smallest_subarray.len() - 1] as isize;
                    result = (latest_freq[0] as isize, latest_freq[idx] as isize);

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

        let keywords = Vec::from_iter(["banana".into(),"cat".into()]);
    
        assert_eq!((8,10),smallest_sequentially_covering_subset(arr, keywords));
    }
    }
}