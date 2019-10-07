#[derive(Debug, PartialEq, Clone)]
pub struct Borrower {
    name: String,
    max_books: u8,
}

impl Borrower {
    pub fn new(name: &str, max_books: u8) -> Borrower {
        Borrower {
            name: name.to_owned(),
            max_books,
        }
    }

    pub fn get_name(&self) -> &str {
        &(self.name)
    }

    pub fn get_max_books(&self) -> u8 {
        self.max_books
    }

    pub fn set_name(self, name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..self
        }
    }

    pub fn set_max_books(self, max_books: u8) -> Self {
        Self { max_books, ..self }
    }

    pub fn borrower_to_string(&self) -> String {
        format!("{} {}{} {}", &(self.name), "(", &(self.max_books), "books)")
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_borrower_construct() {
        let br1 = Borrower::new("Borrower1", 1);

        assert_eq!(Borrower::get_name(&br1), "Borrower1");
        assert_eq!(Borrower::get_max_books(&br1), 1);
    }

    #[test]
    fn test_set_name() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower {
            name: "Borrower2".to_owned(),
            max_books: 1,
        };
        assert_eq!(Borrower::set_name(br1.clone(), "Borrower2"), br2);
    }

    #[test]
    fn test_set_max_books() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower {
            name: "Borrower1".to_owned(),
            max_books: 2,
        };
        assert_eq!(Borrower::set_max_books(br1.clone(), 2), br2);
    }

    #[test]
    fn test_test_to_string() {
        let br1 = Borrower::new("Borrower1", 1);
        assert_eq!(br1.borrower_to_string(), "Borrower1 (1 books)");
    }
}
