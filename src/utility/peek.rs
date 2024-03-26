/*
    Iterator that allows you to lookahead a certain number of times
*/

// nobody cares about the warnings
#![allow(dead_code, unused_imports)]

pub struct Peek<const N: usize, I: Iterator> {
    iter: I,
    buffer: [Option<I::Item>; N], //buffer stores N lookaheads
}

impl<const N: usize, I: Iterator> Peek<N, I> {
    //construct and fill the buffer with N items from the underlying iterator
    pub fn from(iter: I) -> Self {
        let mut peek = Self {
            iter,
            buffer: std::array::from_fn(|_| None),
        };
        for _ in 0..N {
            peek.consume();
        } //prime the underlying buffer
        peek
    }

    //peek an element in the buffer, if it exists
    //maybe we should return an error if u index out of bounds? std get returns a none
    pub fn peek(&self, n: usize) -> Option<&I::Item> {
        self.buffer.get(n)?.as_ref()
    }

    //push underlying iter.next() into buffer
    fn consume(&mut self) -> Option<I::Item> {
        let leftmost = self.buffer.first_mut()?.take(); //take leftmost element
        self.buffer.rotate_left(1); //shift elements to the left
        *self.buffer.last_mut()? = self.iter.next(); //move iter output into rightmost element
        leftmost
    }
}

//make peek an iterator
impl<const N: usize, I: Iterator> Iterator for Peek<N, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn peek() {
        let string = String::from("hello");
        let mut iter: Peek<3, _> = Peek::from(string.chars());

        //get prefix by looking ahead 3
        let prefix = |i: &Peek<3, _>| {
            (0..3)
                .into_iter()
                .map(|n| i.peek(n).unwrap())
                .collect::<String>()
        };

        //prefix tests
        assert_eq!(prefix(&iter), String::from("hel"));
        assert_eq!(iter.next(), Some('h'));
        assert_eq!(prefix(&iter), String::from("ell"));
        assert_eq!(iter.next(), Some('e'));
        assert_eq!(prefix(&iter), String::from("llo"));

        //out of bounds peek
        assert_eq!(iter.peek(4), None);

        //exhaust peek
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.peek(3), None);

        //exhaust underlying iter
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), None);
    }
}
