pub use crate::borrower::Borrower;

#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub borrower: Option<Borrower>,
}

impl Book {
    pub fn new(title: &str, author: &str) -> Book {
        Book { title: title.to_owned(), author: author.to_owned(), borrower: None }
    }
    pub fn set_title(&mut self, title: &str) { self.title = title.to_owned() }
    pub fn set_author(&mut self, author: &str) { self.author = author.to_owned() }
    pub fn set_borrower(&mut self, borrower: Option<Borrower>) { self.borrower = borrower }

    pub fn available_string(&self) -> String {
        match &self.borrower {
            Some(br) => format!("{} {}", "Checked out to", &(br.name)),
            None => "Available".to_owned()
        }
    }

//    pub fn borrower_to_string(&self) -> String {
//        let mut bs: String = self.name.to_owned() + " (";
//        let mb: String = self.max_books.to_owned();
//        bs.push_str(&mb);
//        bs += " books)"; // an alternate way to concatenate
//        bs
//    }
}

#[test]
fn test_new() {
    let bk = Book {
        title: "title1".to_owned(),
        author: "author1".to_owned(),
        borrower: None,
    };
    assert_eq!(bk, Book::new("title1", "author1"));
}

#[test]
fn test_set_title() {
    let mut bk1 = Book {
        title: "title1".to_owned(),
        author: "author1".to_owned(),
        borrower: None,
    };
    bk1.set_title("title2");
    let bk2 = Book {
        title: "title2".to_owned(),
        author: "author1".to_owned(),
        borrower: None,
    };
    assert_eq!(bk2, bk1);
}

#[test]
fn test_set_author() {
    let mut bk1 = Book {
        title: "title1".to_owned(),
        author: "author1".to_owned(),
        borrower: None,
    };
    bk1.set_author("author2");
    let bk2 = Book {
        title: "title1".to_owned(),
        author: "author2".to_owned(),
        borrower: None,
    };
    assert_eq!(bk2, bk1);
}

#[test]
fn test_set_borrower() {
    let mut bk1 = Book {
        title: "title1".to_owned(),
        author: "author1".to_owned(),
        borrower: None,
    };
    bk1.set_borrower(Some(Borrower
        {
            name: "borrower1".to_owned(),
            max_books: 1,
        }));
    let bk2 = Book {
        title: "title1".to_owned(),
        author: "author1".to_owned(),
        borrower: Some(Borrower
            {
                name: "borrower1".to_owned(),
                max_books: 1,
            }),
    };
    assert_eq!(bk2, bk1);
}

#[test]
fn test_book_to_string() {
    let bk1 = Book {
        title: "title1".to_owned(),
        author: "author1".to_owned(),
        borrower: None,
    };
    assert_eq!(bk1.available_string(), "Available");

    let bk2 = Book {
        title: "title1".to_owned(),
        author: "author1".to_owned(),
        borrower: Some(Borrower
            {
                name: "borrower1".to_owned(),
                max_books: 1,
            }),
    };
    assert_eq!(bk2.available_string(), "Checked out to borrower1");
}
