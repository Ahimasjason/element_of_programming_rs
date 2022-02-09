



struct CircularQueue<T >{

    entries : Vec<Option<T>>, 
    head : usize,
    tail : usize,
    size : usize,
    cap : usize,
}

impl<T:Copy + Clone + std::fmt::Debug> CircularQueue<T>{

    const FACTOR: usize = 2;

    fn new( cap: usize) -> Self {

        let arr = vec![None; cap];

        Self {
            entries : arr,
            head : 0,
            tail : 0,
            size: 0,
            cap
        }

    }

    fn enqueue(&mut self, data: T){

        if self.size == self.entries.len() {

            let start = &self.entries[self.head..];
            let end = &self.entries[..self.head];
            eprintln!(" start -> {:?}", start);
            eprintln!(" end -> {:?}", end);
            let  s_len = start.len();
            let  e_len = end.len();
            let mut new = vec![None; self.entries.len() * Self::FACTOR];
            new[0..s_len].copy_from_slice(start);
            new[s_len..(s_len + e_len)].copy_from_slice(end);
            
            self.entries = new;
            self.head = 0;
            self.tail = self.size;
        }
        self.entries[self.tail] = Some(data);
        self.tail = (self.tail + 1) % self.entries.len();
        self.size += 1;
    }


    fn dequeue(&mut self) -> Option<T>{

        if self.size == 0 {return None}

        self.size -= 1;
        let x = self.entries[self.head].take();
        self.head = (self.head + 1) % self.entries.len();

        x
    }

}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_circular_queue(){
        let mut q = CircularQueue::<usize>::new(1);

        // new.enqueue(1);
        // eprintln!("{:?}",new.entries);
        // new.enqueue(2);
        // eprintln!("{:?}",new.entries);
        // new.enqueue(3);
        // eprintln!("{:?}",new.entries);
        // new.dequeue();
        // new.enqueue(4);
        // eprintln!("{:?}",new.entries);
        // new.enqueue(5);
        // eprintln!("{:?}",new.entries);
        // new.enqueue(6);
        // eprintln!("{:?}",new.entries);

        for i in 0..10{
            q.enqueue(i);
        }
        assert_eq!(q.size, 10);

        for i in 0..10{
            assert_eq!(q.dequeue(), Some(i));
        }
    }
}