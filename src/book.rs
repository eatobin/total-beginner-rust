use crate::borrower::Borrower;

#[derive(Debug, PartialEq, Clone)]
pub struct Book<'a> {
    title: String,
    author: String,
    borrower: Option<&'a Borrower>,
}

impl<'a> Book<'a> {
    pub fn new(title: &str, author: &str, borrower: Option<&'a Borrower>) -> Book<'a> {
        Book {
            title: title.to_owned(),
            author: author.to_owned(),
            borrower,
        }
    }

//    pub fn get_title(&self) -> &str {
//        &(self.title)
//    }
//
//    pub fn set_title(self, title: &str) -> Self {
//        Self {
//            title: title.to_owned(),
//            ..self
//        }
//    }
//
//    pub fn get_author(&self) -> &str {
//        &(self.author)
//    }
//
//    pub fn set_author(self, author: &str) -> Self {
//        Self {
//            author: author.to_owned(),
//            ..self
//        }
//    }
//
//    pub fn get_borrower(&self) -> Option<&Borrower> {
//        self.borrower
//    }
//
//    pub fn set_borrower(self, borrower: Option<&'a Borrower>) -> Self {
//        Self { borrower, ..self }
//    }

//    pub fn available_string(&self) -> String {
//        match &self.borrower {
//            Some(br) => format!("{} {}", "Checked out to", br.get_name()),
//            None => "Available".to_owned(),
//        }
//    }
//
//    pub fn book_to_string(&self) -> String {
//        format!(
//            "{} {} {}{} {}",
//            &(self.title),
//            "by",
//            &(self.author),
//            ";",
//            &(self.available_string())
//        )
//    }
}

//#[cfg(test)]
//mod tests {
//    // Note this useful idiom: importing names from outer (for mod tests) scope.
//    use super::*;
//
//    #[test]
//    fn test_book_construct() {
//        let bk1 = Book::new("Title1", "Author1", None);
//        assert_eq!(bk1.get_title(), "Title1");
//        assert_eq!(bk1.get_author(), "Author1");
//        let br1 = Borrower::new("Borrower1", 1);
//        let sbr1 = Some(&br1);
//        let bk2 = Book::new("Title1", "Author1", sbr1);
//        assert_eq!(bk2.get_borrower(), (Some(&Borrower::new("Borrower1", 1))));
//        assert_eq!(bk1.get_borrower(), None);
//    }
//
//    #[test]
//    fn test_book_set_values() {
//        let br1 = Borrower::new("Borrower1", 1);
//        let sbr1 = Some(&br1);
//        let bk1 = Book::new("Title1", "Author1", None);
//        let bk2 = Book::new("Title1", "Author1", sbr1);
//        let bk3 = Book::new("Title2", "Author1", None);
//        assert_eq!(bk1.clone().set_title("Title2"), bk3);
//        let bk4 = Book::new("Title1", "Author2", None);
//        assert_eq!(bk1.clone().set_author("Author2"), bk4);
//        assert_eq!(
//            bk1.clone()
//                .set_borrower(Some(&Borrower::new("Borrower1", 1))),
//            bk2
//        );
//    }
//
//    #[test]
//    fn test_book_to_string() {
//        let br1 = Borrower::new("Borrower1", 1);
//        let sbr1 = Some(&br1);
//        let bk1 = Book::new("Title1", "Author1", None);
//        let bk2 = Book::new("Title1", "Author1", sbr1);
//        assert_eq!(bk1.available_string(), "Available");
//        assert_eq!(bk2.available_string(), "Checked out to Borrower1");
//        assert_eq!(bk1.book_to_string(), "Title1 by Author1; Available");
//        assert_eq!(
//            bk2.book_to_string(),
//            "Title1 by Author1; Checked out to Borrower1"
//        );
//    }
//}
