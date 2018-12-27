pub use crate::book::Book;
pub use crate::borrower::Borrower;

pub fn add_item<T: PartialEq>(x: T, xs: &mut Vec<T>) {
    if !xs.contains(&x) {
        xs.push(x);
    }
}



// #![allow(unused)]
// fn main() {
//     // let mut char_list = vec!['y', 'm', 'a', 'q'];
//     let mut word_list = vec![&"cat", &"dog"];
//     assert!(word_list.contains(&&"cat"));

//     // println!("{:?}", char_list);
//     // add_item('e', &mut char_list);
//     // println!("{:?}", char_list);
//     // add_item('y', &mut char_list);
//     // println!("{:?}", char_list);
//     add_item(&"bird", &mut word_list);
//     println!("{:?}", &word_list);
// }

// pub fn add_item<'a, T: PartialEq>(x: &'a T, xs: &'a mut Vec<&'a T>) {
//     if !xs.contains(&x) {
//         xs.push(x);
//     }
// }
