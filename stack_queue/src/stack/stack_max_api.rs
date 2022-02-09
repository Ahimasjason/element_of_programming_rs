



#[derive(Debug)]
struct MaxWithCount<T>(T, usize);


struct Stack <T> {
    elements : Vec<T>,
    _cached_max : Vec<MaxWithCount<T>>
}


impl <T> Stack <T> 

where T: Copy + Ord + PartialEq + std::fmt::Debug
{


    fn new() -> Self {
        Self {elements: vec![], _cached_max: vec![]}
    }
    /*
     * Insert Operation
     * 1) Insert data to the elements
     * 2) if elements len is 1 then `_cached_max` is data and `count` is 1
     * 3) else check if last max index less than current elements then add to _cached_max
     * 
     * */ 
    pub fn insert(&mut self, data: T){
        self.elements.push(data);
        // check if cached_element is null
        if self._cached_max.len() == 0{
            self._cached_max.push(
                MaxWithCount(data, 1)
            );
        } else {
            let len = self._cached_max.len() - 1;
            let curr_max = &mut self._cached_max[len];

            if data ==  curr_max.0{
                
                curr_max.1 += 1;
            } else if data > curr_max.0 {
                self._cached_max.push(
                    MaxWithCount(data, 1)
                ) 
            }
        }
    }


    /*
     * Check for is elements is empty
     * If it is not empty pop the elements
     * if element is equal to max:
     *      if count == 0 the pop the element 
     *      else decrement the counter
     * 
     *  
    **/

    pub fn pop(&mut self) -> Option<T> {
        if self.empty(){
            return None
        }
        let element = self.elements.pop();
        
        let len = self._cached_max.len() - 1;
        let curr_max = &mut self._cached_max[len];        
        if *element.as_ref().unwrap() == curr_max.0{
            //  if the element is count is one than pop
            if curr_max.1 == 1 {
                //  pop the count
                self._cached_max.pop();
            } // decrease the count
            else {
                curr_max.1 -= 1;
            }
        }
        element
    }

    pub fn empty(&self) -> bool {
        self.elements.is_empty()
    }
}





#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_insert(){
        let mut stack = Stack::new();
        let e = [2,2,1,4,5,5,3];
        for i in e{
            stack.insert(i);
        }
        eprintln!("stack -> {:?}", stack.elements);
        eprintln!("stack -> {:?}", stack._cached_max);
        

    }
}
