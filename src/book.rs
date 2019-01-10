use crate::borrower::Borrower;

#[derive(Debug, PartialEq)]
pub struct Book {
    title: String,
    author: String,
    borrower: Option<Borrower>,
}

impl Book {
    pub fn new(title: &str, author: &str, borrower: Option<Borrower>) -> Book {
        Book {
            title: title.to_owned(),
            author: author.to_owned(),
            borrower: borrower,
        }
    }

    pub fn get_title(&self) -> &str {
        &(self.title)
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_owned()
    }

    pub fn get_author(&self) -> &str {
        &(self.author)
    }

    pub fn set_author(&mut self, author: &str) {
        self.author = author.to_owned()
    }

    pub fn set_borrower(&mut self, borrower: Option<Borrower>) {
        self.borrower = borrower
    }

    pub fn available_string(&self) -> String {
        match &self.borrower {
            Some(br) => format!("{} {}", "Checked out to", br.get_name()),
            None => "Available".to_owned(),
        }
    }

    pub fn book_to_string(&self) -> String {
        format!(
            "{} {} {}{} {}",
            &(self.title),
            "by",
            &(self.author),
            ";",
            &(self.available_string())
        )
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new_book() {
        let bk = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };
        assert_eq!(bk, Book::new("Title1", "Author1", None));
    }

    #[test]
    fn test_get_title() {
        let bk = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };
        assert_eq!(bk.get_title(), "Title1");
    }

    #[test]
    fn test_set_title() {
        let mut bk1 = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };
        bk1.set_title("Title2");
        let bk2 = Book {
            title: "Title2".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };
        assert_eq!(bk2, bk1);
    }

    #[test]
    fn test_set_author() {
        let mut bk1 = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };
        bk1.set_author("Author2");
        let bk2 = Book {
            title: "Title1".to_owned(),
            author: "Author2".to_owned(),
            borrower: None,
        };
        assert_eq!(bk2, bk1);
    }

    #[test]
    fn test_set_borrower() {
        let mut bk1 = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };
        bk1.set_borrower(Some(Borrower::new("Borrower1", 1)));
        let bk2 = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: Some(Borrower::new("Borrower1", 1)),
        };
        assert_eq!(bk2, bk1);

        bk1.set_borrower(None);
        let bk2 = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };
        assert_eq!(bk2, bk1);
    }

    #[test]
    fn test_book_to_string() {
        let bk1 = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };
        assert_eq!(bk1.available_string(), "Available");

        let bk2 = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: Some(Borrower::new("Borrower1", 1)),
        };
        assert_eq!(bk2.available_string(), "Checked out to Borrower1");

        assert_eq!(bk1.book_to_string(), "Title1 by Author1; Available");
        assert_eq!(
            bk2.book_to_string(),
            "Title1 by Author1; Checked out to Borrower1"
        );
    }
}
