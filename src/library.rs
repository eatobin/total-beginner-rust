//use crate::book::Book;
//use crate::borrower::Borrower;
//use std::collections::HashSet;
//
//pub struct Library {
//    borrowers: HashSet<Borrower>,
//    books: HashSet<Book>,
//}
//
//impl Library {
//    pub fn new() -> Self {
//        Library {
//            borrowers: HashSet::new(),
//            books: HashSet::new(),
//        }
//    }
//
//    pub fn add_borrower(&mut self, borrower: Borrower) -> bool {
//        self.borrowers.insert(borrower)
//    }
//
//    pub fn add_book(&mut self, book: Book) -> bool {
//        self.books.insert(book)
//    }
//
//    // TODO add_item?
//
//    pub fn find_borrower(&self, name: &str) -> Option<&mut Borrower> {
//        self.borrowers
//            .iter_mut()
//            .find(|&br| Borrower::get_name(br) == name)
//    }
//}
//
////pub fn find_borrower<'a>(
////    name: &str,
////    brs: &'a HashSet<Borrower>,
////    f: &Fn(&'a Borrower) -> &str,
////) -> Option<&'a Borrower> {
////    brs.iter().find(|&i| f(i) == name)
////}
//
//#[cfg(test)]
//mod tests {
//    // Note this useful idiom: importing names from outer (for mod tests) scope.
//    use super::*;
//
//    #[test]
//    fn test_add_borrower_and_book() {
//        let mut lib = Library::new();
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower1", 1);
//        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
//        let bk2 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
//        //        let br2 = Borrower::new("Borrower1", 1);
//        //        let br3 = Borrower::new("Borrower1", 1);
//        //        let br4 = Borrower::new("Borrower1", 1);
//        //        let mut brs: HashSet<Borrower> = HashSet::new();
//
//        assert_eq!(lib.add_borrower(br1), true);
//        assert_eq!(lib.add_borrower(br2), false);
//
//        assert_eq!(lib.add_book(bk1), true);
//        assert_eq!(lib.add_book(bk2), false);
//    }
//
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
//}
