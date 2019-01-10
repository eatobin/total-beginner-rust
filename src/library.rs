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

    pub fn get_borrowers(&mut self) -> &mut Vec<Borrower> {
        &mut (self.borrowers)
    }

    pub fn get_books(&mut self) -> &mut Vec<Book> {
        &mut (self.books)
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

    //    pub fn find_borrower(&self, name: &str) -> Option<&mut Borrower> {
    //        self.borrowers
    //            .iter_mut()
    //            .find(|&br| Borrower::get_name(br) == name)
    //    }
}

////pub fn find_borrower<'a>(
////    name: &str,
////    brs: &'a HashSet<Borrower>,
////    f: &Fn(&'a Borrower) -> &str,
////) -> Option<&'a Borrower> {
////    brs.iter().find(|&i| f(i) == name)
////}
//
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_borrower_and_book() {
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

    //    //    #[test]
    //    //    fn test_remove_book() {
    //    //        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
    //    //        let bk2 = Book::new("Title2", "Author2", None);
    //    //        let mut bks1: Vec<&Book> = vec![&bk1, &bk2];
    //    //        let bks2: Vec<&Book> = vec![&bk1, &bk2];
    //    //
    //    //        bks1 = remove_book(&Book::new("Title22", "Author2", None), bks1);
    //    //        assert_eq!(bks1, bks2);
    //    //
    //    //        let bks2: Vec<&Book> = vec![&bk1];
    //    //
    //    //        bks1 = remove_book(&Book::new("Title2", "Author2", None), bks1);
    //    //        assert_eq!(bks1, bks2);
    //    //    }
    //    //
    //    //    #[test]
    //    //    fn test_find_item() {
    //    //        let br1 = Borrower::new("Borrower1", 1);
    //    //        let br2 = Borrower::new("Borrower2", 2);
    //    //        let mut brs: Vec<&Borrower> = Vec::new();
    //    //        brs = add_item(&br1, brs);
    //    //        brs.push(&br2);
    //    //
    //    //        let fnd_br = find_item("Borrower1", &brs, &Borrower::get_name);
    //    //        assert_eq!(fnd_br, Some(&&br1));
    //    //
    //    //        let fnd_br: Option<&&Borrower> = find_item("Borrower11", &brs, &Borrower::get_name);
    //    //        assert_eq!(fnd_br, None);
    //    //
    //    //        let bk1 = Book::new("Title1", "Author1", None);
    //    //        let bk2 = Book::new("Title2", "Author2", None);
    //    //        let mut bks: Vec<&Book> = Vec::new();
    //    //        bks.push(&bk1);
    //    //        bks.push(&bk2);
    //    //
    //    //        let fnd_bk = find_item("Title1", &bks, &Book::get_title);
    //    //        assert_eq!(fnd_bk, Some(&&bk1));
    //    //
    //    //        let fnd_bk = find_item("Title11", &bks, &Book::get_title);
    //    //        assert_eq!(fnd_bk, None);
    //    //    }
}
