pub use crate::borrower::Borrower;

#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub borrower: Option<Borrower>,
}

impl Book {
    pub fn new(title: &str, author: &str) -> Book {
        Book { title: title.to_string(), author: author.to_string(), borrower: None }
    }
    pub fn set_title(&mut self, title: &str) { self.title = title.to_string() }
    pub fn set_author(&mut self, author: &str) { self.author = author.to_string() }
    pub fn set_borrower(&mut self, borrower: Option<Borrower>) { self.borrower = borrower }
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
    let bk = Book {
        title: "title1".to_string(),
        author: "author1".to_string(),
        borrower: None,
    };
    assert_eq!(bk, Book::new("title1", "author1"));
}

#[test]
fn test_set_title() {
    let mut bk1 = Book {
        title: "title1".to_string(),
        author: "author1".to_string(),
        borrower: None,
    };
    bk1.set_title("title2");
    let bk2 = Book {
        title: "title2".to_string(),
        author: "author1".to_string(),
        borrower: None,
    };
    assert_eq!(bk2, bk1);
}

#[test]
fn test_set_author() {
    let mut bk1 = Book {
        title: "title1".to_string(),
        author: "author1".to_string(),
        borrower: None,
    };
    bk1.set_author("author2");
    let bk2 = Book {
        title: "title1".to_string(),
        author: "author2".to_string(),
        borrower: None,
    };
    assert_eq!(bk2, bk1);
}
