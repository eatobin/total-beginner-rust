pub use crate::borrower::Borrower;

#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub borrower: Option<Borrower>,
}

impl Book {
    pub fn new(title: &str, author: &str, borrower: Option<Borrower>) -> Book {
        Book { title: title.to_string(), author: author.to_string(), borrower }
    }
//    pub fn set_name(&mut self, name: &str) { self.name = name.to_string() }
//    pub fn set_max_books(&mut self, mb: u8) {
//        self.max_books = mb;
//    }
//
//    pub fn borrower_to_string(&self) -> String {
//        let mut bs: String = self.name.to_string() + " (";
//        let mb: String = self.max_books.to_string();
//        bs.push_str(&mb);
//        bs += " books)"; // an alternate way to concatenate
//        bs
//    }
}

#[test]
fn test_new() {
    let bk1 = Book {
        title: "title1".to_string(),
        author: "author1".to_string(),
        borrower: Some(Borrower
            {
                name: "borrower1".to_string(),
                max_books: 1,
            }),
    };
    let bk2 = Book {
        title: "title1".to_string(),
        author: "author1".to_string(),
        borrower: None,
    };
    assert_eq!(bk1, Book::new("title1", "author1",
                              Some(Borrower::new("borrower1", 1))));
    assert_eq!(bk2, Book::new("title1", "author1", None));
}
