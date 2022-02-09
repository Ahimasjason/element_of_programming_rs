



struct StackQueue<T> {

    enq : Vec<T>,
    deq : Vec<T>,
}


impl<T> StackQueue<T>{


    fn new() -> Self {

        Self {enq :vec![], deq: vec![]}
    }

    fn enqueue(&mut self, data: T){

        self.enq.push(data);
    }

    fn dequeue(&mut self) -> Option<T>{
        
        if self.deq.len() == 0{

            while !self.enq.is_empty(){
                self.deq.push(self.enq.pop().unwrap());
            }
        }
        self.deq.pop()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_stack(){

        let mut s = StackQueue::new();

        for i in 0..1000{
            s.enqueue(i);
        }


        for j in 0..1000{
            assert_eq!(s.dequeue(), Some(j));
        }
    }
}