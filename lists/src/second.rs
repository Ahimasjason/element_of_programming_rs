// #[derive(Debug)]
// enum Link<T> {
//     // internals of enum totally public
//     Empty,
//     More(Box<Node<T>>),
// }

type Link<T> = Option<Box<Node<T>>>;

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
        Self { head: None }
    }

    pub fn push(&mut self, val: T) {
        let n = Node {
            elem: val,
            next: self.head.take(), // take the option and leave the none in its placec
        };
        self.head = Some(Box::new(n))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        eprintln!("Drop is getting called ! on node");
        let mut curr_node = self.head.take();

        while let Some(mut boxed_node) = curr_node {
            curr_node = boxed_node.next.take(); // take will assign None in its place
        }
    }
}

/// life time is a name of a region or scope
/// when it is reference is tagged with life time we are saying that it has to be valid for the
// entire region
//fn foo<'a>(&'a A) -> &'a B means input lives atleast aslong as output
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    // creates the iter. Now &self needs to be valid as long as the
    // Iter is around.
    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {

    //             // Iter {next: self.head.as_deref()}

    //     Iter {next: self.head.as_ref().map::<&Node<T>,_>(|n| &n)}
    // }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(), // Converts from Option<T> (or &Option<T>) to Option<&T::Target>
        }

        // Iter {next: self.head.as_ref().map::<&Node<T>,_>(|n| &n)}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref(); // using map to take reference to the item
            &n.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.next.as_deref_mut();
            &mut n.elem
        })
    }
}

impl<T> List<T> {
    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter actually yields the pointer to the item
// so idea is keep the head node pointer

mod test {

    use super::*;

    #[test]
    fn peek() {
        let mut list = List::<i32>::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42;
        });
        assert_eq!(list.peek(), Some(&42));
    }

    #[test]
    fn test_second_list() {
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
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        // eprintln!("peeking on the list {:?}");
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        eprintln!("list head is after --------------------------> {:?}", list);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));

        eprintln!(
            "list head is after on iter_mut --------------------------> {:?}",
            list
        );
    }
}
