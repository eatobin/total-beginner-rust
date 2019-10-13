use crate::borrower::Borrower;

#[derive(Debug, PartialEq, Clone)]
pub struct Book<'a> {
    pub title: String,
    pub author: String,
    pub borrower: Option<&'a Borrower>,
}

impl<'a> Book<'a> {
    pub fn new(title: &str, author: &str, borrower: Option<&'a Borrower>) -> Book<'a> {
        Book {
            title: title.to_owned(),
            author: author.to_owned(),
            borrower,
        }
    }

    pub fn set_title(&mut self, title: &str) { self.title = title.to_owned() }

    pub fn set_author(&mut self, author: &str) { self.author = author.to_owned() }

    pub fn set_borrower(&mut self, borrower: Option<&'a Borrower>) { self.borrower = borrower }

    pub fn available_string(&self) -> String {
        match self.borrower {
            Some(br) => format!("{} {}", "Checked out to", br.name),
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
    fn test_book_to_string() {
        let br1 = Borrower::new("Borrower1", 1);
        let sbr1 = Some(&br1);
        let bk1 = Book::new("Title1", "Author1", None);
        let bk2 = Book::new("Title1", "Author1", sbr1);
        assert_eq!(bk1.available_string(), "Available");
        assert_eq!(bk2.available_string(), "Checked out to Borrower1");
        assert_eq!(bk1.book_to_string(), "Title1 by Author1; Available");
        assert_eq!(
            bk2.book_to_string(),
            "Title1 by Author1; Checked out to Borrower1"
        )
    }

    #[test]
    fn test_book_set_values() {
        let br1 = Borrower::new("Borrower1", 1);
        let sbr1 = Some(&br1);
        let bk1 = &mut Book::new("Title1", "Author1", None);
        let bk2 = &mut Book::new("Title2", "Author2", sbr1);
        let bk3 = &mut Book::new("Title2", "Author1", None);
        bk1.set_title("Title2");
        assert_eq!(bk1, bk3);
        let bk4 = &mut Book::new("Title2", "Author2", None);
        bk1.set_author("Author2");
        assert_eq!(bk1, bk4);
        let br2 = Borrower::new("Borrower1", 1);
        let sbr2 = Some(&br2);
        bk1.set_borrower(sbr2);
        assert_eq!(bk1, bk2)
    }
}
