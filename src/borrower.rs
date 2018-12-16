#[derive(Debug, PartialEq)]
pub struct Borrower<'a> {
    name: &'a str,
    max_books: u8,
}

pub fn build_borrower(name: &str, max_books: u8) -> Borrower {
    Borrower { name, max_books }
}

impl<'a> Borrower<'a> {
    pub fn set_name(&mut self, n: &'a str) {
        self.name = n;
    }

    pub fn set_max_books(&mut self, mb: u8) {
        self.max_books = mb;
    }

    pub fn borrower_to_string(&self) -> String {
        let mut bs: String = self.name.to_string() + " (";
        let mb: String = self.max_books.to_string();
        bs.push_str(&mb);
        bs.push_str(" books)");
        bs
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_build_borrower() {
        let br = Borrower {
            name: "borrower1",
            max_books: 1,
        };
        assert_eq!(br, build_borrower("borrower1", 1));
    }

    #[test]
    fn test_set_name() {
        let mut br1 = Borrower {
            name: "borrower1",
            max_books: 1,
        };
        br1.set_name("borrower2");
        let br2 = Borrower {
            name: "borrower2",
            max_books: 1,
        };
        assert_eq!(br2, br1);
    }

    #[test]
    fn test_set_max_books() {
        let mut br1 = Borrower {
            name: "borrower1",
            max_books: 1,
        };
        br1.set_max_books(2);
        let br2 = Borrower {
            name: "borrower1",
            max_books: 2,
        };
        assert_eq!(br2, br1);
    }

    #[test]
    fn test_borrower_to_string() {
        let br1 = Borrower {
            name: "borrower1",
            max_books: 1,
        };
        assert_eq!("borrower1 (1 books)", br1.borrower_to_string());
    }
}
