

use std::fmt;

pub mod top_k;
pub mod merge_sorted_array;
pub mod sort_mountain_array;
pub mod sort_app_sorted;
pub mod online_median;
pub mod max_k;





pub struct Heap<T>{
    data : Vec<T>,
    func : fn(&T,&T) -> bool
}


impl <T> Heap<T> 

    where T:Copy + PartialOrd

{

    pub fn new(op : fn(&T,&T) -> bool ) -> Self {

        Self {data: Vec::new(), func: op}
    }

    pub fn insert(&mut self, data : T) {
        self.data.push(data);
        if self.data.len() == 1{
            return
        }

        self.bubble_up(self.data.len() - 1);
    }

    pub fn top(&mut self) -> Option<T> {

        if self.data.is_empty(){
            return None
        }

        let p = self.data.pop();
        if self.data.is_empty(){ return p};
        let tmp = self.data[0];
        self.data[0] = p.unwrap();
        self.push_down(0);
        Some(tmp)
    }

    fn bubble_up(&mut self, idx : usize) {
         let mut curr_idx = idx;   
         while curr_idx > 0 {
             let parent = (curr_idx - 1) / 2;
             if self.positive(curr_idx, parent){
                 self.data.swap(curr_idx, parent);
                 curr_idx = parent;    
             }else { break; }
         }
    }


    fn positive(&self, left:usize, right:usize) -> bool{

        (self.func)(&self.data[left], &self.data[right])
    }
    
    fn push_down(&mut self, idx: usize){

        let mut parent_idx = idx;
        while parent_idx < self.first_child_idx() {
            let hp_child = self.high_priority_child(parent_idx);

            if self.positive(hp_child,parent_idx){
                self.data.swap(hp_child, parent_idx);
            } else {
                break;
            }
            parent_idx = hp_child;
        }
    }

    fn first_child_idx(&self) -> usize {
        if self.data.len() == 1 {return 0}
        ((self.data.len() - 2)  /2 ) + 1
    }

    fn high_priority_child(&self, curr_idx : usize) -> usize {
        
        let left_idx = (curr_idx * 2) + 1;
        let right_idx = (curr_idx * 2) + 2;

        // if right idx is in boundary check for both indicies

        if right_idx < self.data.len() {
            if (self.func)(&self.data[right_idx] , &self.data[left_idx]){
                return right_idx
            } else {
                return left_idx
            }
        }
        left_idx


    }

    fn heapify(&mut self){

        for i in 0..(self.data.len() - 1 / 2) {
            self.push_down(i);
        }
    }


    fn len(&self) -> usize {
        self.data.len()
    }

    fn peek(&self) -> Option<&T>{
        if self.data.is_empty(){ return None}
        Some(&self.data[0])
    }
    
    
}



impl<T: fmt::Debug> fmt::Display  for Heap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: std::fmt::Debug> fmt::Debug for Heap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Heap")
         .field("data", &self.data)
         .finish()
    }
}




#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn test_heap() {
        let mut heap = Heap::new(std::cmp::PartialOrd::gt);
        heap.insert(2);
        println!("{:?}", heap);
        heap.insert(3);
        println!("{:?}", heap);
        heap.insert(5);
        println!("{:?}", heap);
    }

    #[test]
    fn test_push_down(){

        /* 
         *         1
         *        /  \
         *       4    2
         *      /
         *     5
         * */ 
        let mut heap = Heap{data: vec![1,4,5,2],func: std::cmp::PartialOrd::gt};

        heap.push_down(0);
        assert_eq!(heap.data[0], 5);

        let mut heap = Heap{data: vec![1,4,5,2],func: std::cmp::PartialOrd::gt};

        heap.push_down(0);
        assert_eq!(heap.data[0], 5);
        
    }

    #[test]
    fn test_top(){

        let mut heap = Heap{data: vec![], func: std::cmp::PartialOrd::gt};

        for i in [1,4,5,2]{
            heap.insert(i);
        }
        println!("{:?}", heap);
        assert_eq!(heap.top(), Some(5));
        println!("{:?}", heap);
        assert_eq!(heap.top(), Some(4));
        println!("{:?}", heap);
        assert_eq!(heap.top(), Some(2));
        println!("{:?}", heap);
        assert_eq!(heap.top(), Some(1));
        println!("{:?}", heap);
        assert_eq!(heap.top(), None);
    }


    #[test]
    fn test_heapify(){

        let mut heap = Heap{data: vec![1,4,5,2], func:std::cmp::PartialOrd::gt};
        heap.heapify();
        heap.top();
        println!("{:?}", heap);
    }
}
