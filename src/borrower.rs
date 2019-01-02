#[derive(Debug, PartialEq)]
pub struct Borrower {
    pub name: String,
    pub max_books: u8,
}

impl Borrower {
    pub fn new(name: &str, max_books: u8) -> Borrower {
        Borrower {
            name: name.to_owned(),
            max_books,
        }
    }

    //pub fn get_name(br: &Borrower) -> &str {
    //    &(br.name)
    //}

    pub fn get_name(&self) -> &str {
        &(self.name)
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned()
    }

    pub fn set_max_books(&mut self, mb: u8) {
        self.max_books = mb
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
    fn test_new_borrower() {
        let br = Borrower {
            name: String::from("Borrower1"),
            max_books: 1,
        };
        assert_eq!(br, Borrower::new("Borrower1", 1));
    }

    #[test]
    fn test_get_name() {
        let br = Borrower {
            name: "Borrower1".to_owned(),
            max_books: 1,
        };
        assert_eq!(Borrower::get_name(&br), "Borrower1");
    }

    #[test]
    fn test_set_name() {
        let mut br1 = Borrower {
            name: "Borrower1".to_owned(),
            max_books: 1,
        };
        br1.set_name("borrower2");
        let br2 = Borrower {
            name: "borrower2".to_owned(),
            max_books: 1,
        };
        assert_eq!(br2, br1);
    }

    #[test]
    fn test_set_max_books() {
        let mut br1 = Borrower {
            name: "Borrower1".to_owned(),
            max_books: 1,
        };
        br1.set_max_books(2);
        let br2 = Borrower {
            name: "Borrower1".to_owned(),
            max_books: 2,
        };
        assert_eq!(br2, br1);
    }

    #[test]
    fn test_borrower_to_string() {
        let br1 = Borrower {
            name: "Borrower1".to_owned(),
            max_books: 1,
        };
        assert_eq!("Borrower1 (1 books)", br1.borrower_to_string());
    }
}
