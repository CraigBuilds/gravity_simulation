use std::slice::Iter;
use std::iter::Chain;

pub struct NextWithRestIterator<'slice, T> {
    slice: &'slice mut [T],
    index: usize,
}

impl<'slice, T> NextWithRestIterator<'slice, T> {
    pub fn new(slice: &'slice mut [T]) -> Self {
        NextWithRestIterator {
            slice,
            index: 0,
        }
    }
}

pub trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

impl<'slice, T> LendingIterator for NextWithRestIterator<'slice, T> {
    type Item<'a> = (&'a mut T, Chain<Iter<'a, T>, Iter<'a, T>>) where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.index < self.slice.len() {
            let (preceding, subsequent) = self.slice.split_at_mut(self.index);
            let (current_value, subsequent) = subsequent.split_first_mut().unwrap();
            let chain_of_rest = preceding.iter().chain(subsequent.iter());
            self.index += 1;
            Some((current_value, chain_of_rest))
        } else {
            None
        }
    }
}

pub trait IntoNextWithRestIterator<'a, T> {
    fn iter_with_rest(self) -> NextWithRestIterator<'a, T>;
}

impl<'a, T> IntoNextWithRestIterator<'a, T> for &'a mut [T] {
    fn iter_with_rest(self) -> NextWithRestIterator<'a, T> {
        NextWithRestIterator::new(self)
    }
}

#[test]
fn test() {
    let mut items = vec![1, 2, 3, 4, 5];

    let mut iter = items.iter_with_rest();
    let mut test_result = String::new();
    while let Some((item, others)) = iter.next() {
        test_result.push_str(&format!("{} {:?}\n", item, others.collect::<Vec<_>>()));
    }
    let expected = "\
    1 [2, 3, 4, 5]\n\
    2 [1, 3, 4, 5]\n\
    3 [1, 2, 4, 5]\n\
    4 [1, 2, 3, 5]\n\
    5 [1, 2, 3, 4]\n";
    assert_eq!(test_result, expected);
}