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

    pub fn set_name(&mut self, name: &str) { self.name = name.to_owned() }

    pub fn set_max_books(&mut self, max_books: u8) { self.max_books = max_books }

    pub fn borrower_to_string(&self) -> String {
        format!("{} {}{} {}", &(self.name), "(", &(self.max_books), "books)")
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_test_to_string() {
        let br1 = Borrower::new("Borrower1", 1);
        assert_eq!(br1.borrower_to_string(), "Borrower1 (1 books)");
    }

    #[test]
    fn test_set_values() {
        let br1 = &mut Borrower::new("Borrower1", 1);
        let br2 = &mut Borrower {
            name: "Borrower2".to_owned(),
            max_books: 1,
        };
        br1.set_name("Borrower2");
        assert_eq!(br1, br2);
        let br3 = &mut Borrower {
            name: "Borrower2".to_owned(),
            max_books: 2,
        };
        br1.set_max_books(2);
        assert_eq!(br1, br3);
    }
}
