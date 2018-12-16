mod borrower {
    #[derive(Debug)]
    pub struct Borrower<'a> {
        name: &'a str,
        max_books: u8,
    }

    pub fn build_borrower(name: &str, max_books: u8) -> Borrower {
        Borrower { name, max_books }
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    impl<'a> Borrower<'a> {
        pub fn set_name(&mut self, n: &'a str) {
            self.name = n;
        }

        pub fn set_max_books(&mut self, mb: u8) {
            self.max_books = mb;
        }

        // fn borrower_to_string(&self) -> &str {
        //     &(self.name.to_owned() + " (" + self.max_books.to_string() + " books)")
        // }
    }
}
fn main() {
    let mut b1 = borrower::build_borrower("me", 66);
    println!("{:?}", b1);
    b1.set_name("newish");
    println!("{:?}", b1);
    b1.set_max_books(88);
    println!("{:?}", b1);
}

// #[cfg(test)] // only compiles when running tests
// use super::Borrower::build_borrower; // import root greet function
// mod tests {
//     #[test]
//     fn test_greet() {

//         assert_eq!("Hello, world!", Borrower::build_borrower("me", 66));
//     }
// }
