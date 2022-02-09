type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

impl<T> List <T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    fn push(&mut self, elem: T){

        let mut new_node = Box::new(
            Node {elem: elem, next: None}
        );


        let new_tail: *mut _ = &mut *new_node;


        if self.head.is_none(){
            self.head = Some(new_node);
            self.tail = new_tail;    
        } else {
            unsafe {
                (*self.tail).next = Some(new_node);
            }
            
            self.tail = new_tail
        }
    }

    fn pop(&mut self) -> Option<T> {

        self.head.take().map(|node| {
            let head = *node;
            self.head = head.next;

            if self.head.is_none(){
                self.tail = std::ptr::null_mut();
            }
            head.elem
        })
    }

    fn peek(&self) -> Option<&T>{
        self.head.as_ref().map(|n| &n.elem)
    }
    fn peek_mut(&mut self) -> Option<&mut T>{
        self.head.as_mut().map(|n| &mut n.elem)
    }
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T>  Drop for List<T> {

    fn drop(&mut self) {

        let mut head_node = self.head.take();

        while let Some(mut curr_node) = head_node{

            head_node = curr_node.next.take();
        }
    }
}


struct IntoIter<T>(List<T>);
struct Iter<'a, T> {next: Option<&'a Node<T>>}

struct IterMut<'a, T> {next: Option<&'a mut  Node<T>>}



impl <T> List <T> {

    fn iter<'a>(&'a mut self) -> Iter<'a, T> {
        Iter{next: self.head.as_deref()}
    }

    fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {

        IterMut{next: self.head.as_deref_mut()}
    }
}

impl<T> Iterator for IntoIter<T> {


    type Item = T ;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {


    type Item = &'a T ;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {


    type Item = &'a mut  T ;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}



#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
