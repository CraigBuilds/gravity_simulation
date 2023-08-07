// use std::fmt::Debug;
// use std::slice::{Iter, IterMut};
// use std::iter::Chain;
// use std::iter::Skip;

// ///Apply the function f to each element of the slice and the iterator of the remaining elements
// fn cross<T>(slice: &mut [T], f: fn (&mut T,  Chain<Iter<'_,T>, Iter<'_,T>>)) {
//     for i in 0..slice.len() {
//         let (preceding, subsequent) = slice.split_at_mut(i);
//         let (current_entity, subsequent) = subsequent.split_first_mut().unwrap();
//         let iter = preceding.iter().chain(subsequent.iter());
//         f(current_entity, iter);
//     }
// }

// #[test]
// fn test_cross() {
//     let mut testvec = vec![1, 2, 3, 4, 5];
//     cross(&mut testvec, |elem, rest| {
//         println!("{} {:?}", elem, rest.collect::<Vec<_>>());
//     });
// }

// fn loops<T: Debug>(slice: &mut [T], f: fn (&mut T, &mut T)) {
//     for i in 0..slice.len() {
//         for j in i + 1..slice.len() {
//             let aptr = &mut slice[i] as *mut T;
//             let bptr = &mut slice[j] as *mut T;
//             // SAFETY: aptr and bptr are different pointers to elements of the slice
//             unsafe {
//                 f(&mut *aptr, &mut *bptr);
//             }
//         }
//     }
// }

// #[test]
// fn test_loops() {
//     let mut testvec = vec![1, 2, 3, 4, 5];
//     loops(&mut testvec, |a, b| {
//         println!("{} {}", a, b);
//     });
// }

// struct LoopsIterator<'a, T> {
//     slice: &'a mut [T],
//     i: usize,
//     j: usize,
// }

// impl<'a, T> LoopsIterator<'a, T> {
//     fn new(slice: &'a mut [T]) -> Self {
//         LoopsIterator {
//             slice,
//             i: 0,
//             j: 1,
//         }
//     }
// }

// impl<'a, T> Iterator for LoopsIterator<'a, T> {
//     type Item = (&'a mut T, &'a mut T);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.i < self.slice.len() {
//             if self.j < self.slice.len() {
//                 let aptr = &mut self.slice[self.i] as *mut T;
//                 let bptr = &mut self.slice[self.j] as *mut T;
//                 // SAFETY: aptr and bptr are different pointers to elements of the slice
//                 unsafe {
//                     let result = Some((&mut *aptr, &mut *bptr));
//                     self.j += 1;
//                     result
//                 }
//             } else {
//                 self.i += 1;
//                 self.j = self.i + 1;
//                 self.next()
//             }
//         } else {
//             None
//         }
//     }
// }

// #[test]
// fn test_loops_iterator() {
//     let mut testvec = vec![1, 2, 3, 4, 5];
//     let mut iter = LoopsIterator::new(&mut testvec);
//     while let Some((a, b)) = iter.next() {
//         println!("{} {}", a, b);
//     }
// }

// // struct LoopsIterator2<'a, T> {
// //     iters: Vec<(&'a mut T, Skip<Iter<'a, T>>)>,
// // }


// // impl<'a, T> LoopsIterator2<'a, T> {
// //     fn new(slice: &'a mut [T]) -> Self {
// //         let mut iters = Vec::new();
// //         for i in 0..slice.len() {
// //             for _ in i + 1..slice.len() {
// //                 // let rest = slice.iter().skip(i); //imutable borrow here
// //                 // let item = &mut slice[i]; //mutable borrow here
// //                 // iters.push((item, rest));
// //                 // We can safely mutably borrow the slice twice, because the two borrows are disjoint
// //                 let ptr = slice.as_mut_ptr();
// //                 unsafe {
// //                     let item = &mut *ptr.add(i);
// //                     let rest = slice.iter().skip(i);
// //                     iters.push((item, rest));
// //                 }
// //             }
// //         }
// //         LoopsIterator2{
// //             iters,
// //         }
// //     }
// // }

// // impl<'a, T> Iterator for LoopsIterator2<'a, T> {
// //     type Item = (&'a mut T, Skip<Iter<'a, T>>);

// //     fn next(&mut self) -> Option<Self::Item> {
// //         self.iters.pop()
// //     }
// // }
