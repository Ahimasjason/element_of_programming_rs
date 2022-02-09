

use arrayvec::{ArrayVec};


pub struct Entry<T>
{
    data : T,
    prev : Option<usize>,
    next : Option<usize>,
}


pub struct LRU<T,const U : usize>
{
    entries : ArrayVec<Entry<T>,U>,
    head: usize,
    tail : usize,
    length: usize,
}






