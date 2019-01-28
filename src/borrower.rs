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
    fn test_borrower() {
        let br1 = Borrower::new("Borrower1", 1);

        // test construct
        assert_eq!(Borrower::get_name(&br1), "Borrower1");
        assert_eq!(br1.get_max_books(), 1);

        // test set_name
        let br2 = Borrower {
            name: "Borrower2".to_owned(),
            max_books: 1,
        };
        assert_eq!(br1.clone().set_name("Borrower2"), br2);

        // test set max_books
        let br2 = Borrower {
            name: "Borrower1".to_owned(),
            max_books: 2,
        };
        assert_eq!(br1.clone().set_max_books(2), br2);

        // test to_string
        assert_eq!(br1.borrower_to_string(), "Borrower1 (1 books)");
    }
}
