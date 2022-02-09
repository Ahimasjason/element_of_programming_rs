#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
// use std::collections::hash_map::Entry;

pub mod anagrams;
pub mod palindromic_permutation;
pub mod constructible;
pub mod lru;
pub mod lca;
pub mod nearest_repeated_words;
pub mod smallest_subarray;
pub mod smallest_seq_order;
pub mod longest_range;





#[derive(Debug)]
pub struct Counter<T>{
    counter : HashMap<T, usize>,
}

impl<T> Counter <T> 
    where 
        T : Hash + Eq
{
    pub fn new() -> Self 
    {
        Self{counter: HashMap::new()}
    }

    pub fn insert(&mut self, key : T)
    {
        *self.counter.entry(key).or_default() += 1;

    }

    pub fn remove(&mut self, key: T) -> Option<usize>
    {

        self.counter.remove(&key)
    }


    pub fn get(&self, key: T) -> Option<&usize>
    {
        self.counter.get(&key)
    }
}



#[derive(Debug)]
pub struct DefaultDict<T, U> 
{
    data : HashMap<T, U>
}


// impl <T, U> DefaultDict <T, U>
//     where 
//         T : Hash + Eq,
//         U : Default,

// {
//     pub fn new() -> Self
//     {
//         Self {data: HashMap::new()}
//     }


//     pub fn insert(&mut self, key : T)
//     {
//         self.data.entry(key).or_default();
//     }
// }




#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_counter() {
        let string = "ABCDASCDRFCS";
        let mut c = Counter::new();

        for ch in string.chars()
        {
            c.insert(ch);
        }
        

        assert_eq!(c.get('A'), Some(&2));

    }
}
