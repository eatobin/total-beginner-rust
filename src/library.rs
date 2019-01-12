use crate::book::Book;
use crate::borrower::Borrower;

#[derive(Debug)]
pub struct Library {
    borrowers: Vec<Borrower>,
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Library {
            borrowers: Vec::new(),
            books: Vec::new(),
        }
    }

    pub fn get_borrowers(&self) -> &Vec<Borrower> {
        &(self.borrowers)
    }

    pub fn get_books(&self) -> &Vec<Book> {
        &(self.books)
    }

    pub fn add_borrower(&mut self, br: Borrower) {
        if !self.borrowers.contains(&br) {
            self.borrowers.push(br);
        }
    }

    pub fn add_book(&mut self, bk: Book) {
        if !self.books.contains(&bk) {
            self.books.push(bk);
        }
    }

    // TODO add_item?

    pub fn find_borrower(&mut self, name: &str) -> Option<&mut Borrower> {
        self.borrowers
            .iter_mut()
            .find(|br| Borrower::get_name(br) == name)
    }

    pub fn find_book(&mut self, title: &str) -> Option<&mut Book> {
        self.books
            .iter_mut()
            .find(|bk| Book::get_title(bk) == title)
    }

    // TODO find_item?

    pub fn get_books_for_borrower(&self, br: &Option<Borrower>) -> Vec<Book> {
        let mut bks_clone = self.books.clone();
        bks_clone.retain(|bk| bk.get_borrower() == br);
        bks_clone
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_borrower_or_book() {
        let mut lib = Library::new();
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower1", 1);
        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let bk2 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));

        assert_eq!(lib.get_borrowers().len(), 0);
        lib.add_borrower(br1);
        assert_eq!(lib.get_borrowers().len(), 1);

        lib.add_borrower(br2);
        assert_eq!(lib.get_borrowers().len(), 1);

        assert_eq!(lib.get_books().len(), 0);
        lib.add_book(bk1);
        assert_eq!(lib.get_books().len(), 1);

        lib.add_book(bk2);
        assert_eq!(lib.get_books().len(), 1);
    }

    #[test]
    fn test_find_borrower_or_book() {
        let mut lib = Library::new();
        let mut br1 = Borrower::new("Borrower1", 1);
        let mut br2 = Borrower::new("Borrower1", 1);
        let mut bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let mut bk2 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        lib.add_borrower(br1);
        lib.add_borrower(br2);
        lib.add_book(bk1);
        lib.add_book(bk2);

        let fnd_br = lib.find_borrower("Borrower1");

        assert_eq!(fnd_br, Some(&mut Borrower::new("Borrower1", 1)));

        let fnd_br = lib.find_borrower("Borrower11");
        assert_eq!(fnd_br, None);

        let fnd_bk = lib.find_book("Title1");
        assert_eq!(
            fnd_bk,
            Some(&mut Book::new(
                "Title1",
                "Author1",
                Some(Borrower::new("Borrower1", 1))
            ))
        );

        let fnd_bk = lib.find_book("Title11");
        assert_eq!(fnd_bk, None);
    }

    #[test]
    fn test_find_books_for_borrower() {
        let mut lib = Library::new();
        let mut br1 = Borrower::new("Borrower1", 1);
        let mut br2 = Borrower::new("Borrower2", 2);
        let mut bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let mut bk2 = Book::new("Title2", "Author2", Some(Borrower::new("Borrower2", 2)));
        let mut bk3 = Book::new("Title3", "Author3", Some(Borrower::new("Borrower2", 2)));
        lib.add_borrower(br1);
        lib.add_borrower(br2);
        lib.add_book(bk1);
        lib.add_book(bk2);
        lib.add_book(bk3);

        let br2_bks = vec![
            Book::new("Title2", "Author2", Some(Borrower::new("Borrower2", 2))),
            Book::new("Title3", "Author3", Some(Borrower::new("Borrower2", 2))),
        ];
        let some_fnd_bks = lib.get_books_for_borrower(&(Some(Borrower::new("Borrower2", 2))));
        assert_eq!(br2_bks, some_fnd_bks);

        let none_fnd_bks = lib.get_books_for_borrower(&(Some(Borrower::new("Borrower22", 2))));
        let no_bks: Vec<Book> = vec![];
        assert_eq!(no_bks, none_fnd_bks);
    }
}
