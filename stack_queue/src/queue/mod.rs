


use std::collections::VecDeque;
pub mod bt_depth_order;
pub mod circular_queue;
pub mod stack_queue;

pub mod queue_with_max;

trait QueueApi {
    type Item;

    fn enqueue(&mut self, data: Self::Item);

    fn dequeue(&mut self) -> Option<Self::Item>;
}

struct Queue<T>(VecDeque<T>);

impl<T> Queue <T> {
    fn new() -> Self {
        Self(VecDeque::new())
    }
} 


impl<T> QueueApi for Queue<T>{
    type Item  = T;
    fn enqueue(&mut self, data: T){
        self.0.push_back(data);
    }

    fn dequeue(&mut self) -> Option<T>{
        self.0.pop_front()
    }

}



#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_queue(){

        let mut queue = Queue::<i32>::new();

        for i in 0..10{
            queue.enqueue(i);
        }

        for i in 0..10{
            assert_eq!(queue.dequeue(), Some(i));
        }
    }
}