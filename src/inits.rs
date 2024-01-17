pub struct Inits<I>
where
    I: Iterator,
{
    data: I,
    buffer: Vec<I::Item>,
}

impl<I> Inits<I>
where
    I: Iterator,
{
    fn new(iter: I) -> Self {
        Inits {
            data: iter,
            buffer: Vec::new(),
        }
    }
}

impl<I> Iterator for Inits<I>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|next| {
            self.buffer.push(next);
            self.buffer.clone()
        })
    }
}

pub trait InitsIterator<A>: Iterator<Item = A> + Sized {
    fn inits(self) -> Inits<Self> {
        Inits::new(self)
    }
}

impl<A, I: Iterator<Item = A>> InitsIterator<A> for I {}

#[cfg(test)]
mod tests {
    use std::vec;

    use pretty_assertions::assert_eq;

    use crate::inits::InitsIterator;

    #[test]
    fn inits_spec() {
        let result: Vec<Vec<char>> = "abcd".chars().inits().collect();
        let expected: Vec<Vec<char>> = vec![
            vec!['a'],
            vec!['a', 'b'],
            vec!['a', 'b', 'c'],
            vec!['a', 'b', 'c', 'd'],
        ];

        assert_eq!(result, expected)
    }
}
