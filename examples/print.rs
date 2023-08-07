use craig_iter::{IntoNextWithRestIterator, LendingIterator};

fn main() {
    let mut items = vec![1, 2, 3, 4, 5];

    let mut iter = items.iter_with_rest();
    while let Some((item, others)) = iter.next() {
        println!("{} {:?}", item, others.collect::<Vec<_>>());
    }
}