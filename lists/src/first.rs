//! `Box` provides the ownership of the allocation and drop their content when they go out of scope

#[derive(Debug)]
enum Link<T> {
    // internals of enum totally public
    Empty,
    More(Box<Node<T>>),
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, val: T) {
        // the reason is self is &mut self but here i am moving self.head to next
        // but i should not move but replace the pointer to it
        //  using std::mem::replce(&mut dest, src) -> dest it will move the src to dest
        let n = Node {
            elem: val,
            next: std::mem::replace(&mut self.head, Link::Empty), // cannot move borrowed content
        };
        self.head = Link::More(Box::new(n));
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(n) => {
                self.head = n.next;
                Some(n.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        eprintln!("Drop is getting called ! on node");
        let mut curr_node = std::mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = curr_node {
            curr_node = std::mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}
mod test {

    use super::*;

    #[test]
    fn test_list() {
        eprintln!(" Test list ");
        let mut list = List::<i32>::new();

        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
