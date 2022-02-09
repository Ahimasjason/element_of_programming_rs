use super::*;

fn top_k(stream: Vec<usize>, k: usize) -> Vec<usize> {
    let mut heap = Heap::<usize>::new(std::cmp::PartialOrd::lt);
    for i in stream {
        if heap.len() == k && *heap.peek().unwrap() < i {
            heap.top();
        }
        if heap.len() < k {
            heap.insert(i);
        }
    }
    heap.data.into_iter().collect::<Vec<usize>>()
}

#[test]
fn test_k(){ 
    let s = vec![1,2,3,4,5,6,7,8,9];
    let r = top_k(s, 2);
    eprintln!("{:?}", r);
    assert_eq!(r, [8,9]);
}