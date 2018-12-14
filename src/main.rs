struct Borrower<'a> {
    name: &'a str,
    max_books: u8,
}

impl<'a> Borrower<'a> {
    fn build_borrower(name: &str, max_books: u8) -> Borrower {
        Borrower {
            name,
            max_books,
        }
    }

    fn set_name(&mut self, n: &'a str) -> () {
        self.name;
    }

    fn set_max_books(&mut self, mb: u8) -> () {
        self.max_books;
    }

    fn borrower_to_string(&self) -> &str {
        &(self.name.to_owned() + " (" + self.max_books.to_string() + " books)")
    }
}

fn main() {
    let b1 = Borrower::build_borrower("me", 66);
    println!("{:?}", b1.name);
}
