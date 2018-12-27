pub use crate::book::Book;
pub use crate::borrower::Borrower;

pub fn add_item<T: PartialEq>(x: T, xs: &mut Vec<T>) {
    if !xs.contains(&x) {
        xs.push(x);
    }
}
