
use std::collections::VecDeque;



struct MaxQueue<T>{
    
    entries : VecDeque<T>,
    // [0] is always max
    max_entries : VecDeque<T>,
}



impl<T: Copy + Ord > MaxQueue <T>{

    fn new() -> Self {
        Self {entries: VecDeque::new(), max_entries: VecDeque::new()}
    }


    fn enqueue(&mut self, data:T){
        self.entries.push_back(data);
        
        while !self.max_entries.is_empty() && self.max_entries[self.max_entries.len() - 1] < data {
            self.max_entries.pop_back();    
        }
        self.max_entries.push_back(data);
    }


    fn dequeue(&mut self) -> Option<T>{

        let data = self.entries.pop_front();

        if !self.max_entries.is_empty(){
            if *data.as_ref().unwrap() == self.max_entries[0]{
                self.entries.pop_front();
            }
        }
    
        data
    }

    fn max(&self) -> Option<T> {
        if !self.max_entries.is_empty(){
            return Some(self.max_entries[0])
        }
        None
        
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_max_queue(){

        let arr = [3,1,3,2,0,1,2,4,4];
        let mut q = MaxQueue::new();
        for i in arr {
            q.enqueue(i);
        }
        assert_eq!(q.max_entries, VecDeque::from([4,4]));
        let arr = [3,1,2,0,1,2];
        let mut q = MaxQueue::new();
        for i in arr {
            q.enqueue(i);
        }
        assert_eq!(q.max_entries, VecDeque::from([3,2,2]));
    }
}