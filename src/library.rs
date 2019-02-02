use crate::book::Book;
use crate::borrower::Borrower;

#[derive(Debug, PartialEq, Clone)]
pub struct Library {
    borrowers: Vec<Borrower>,
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            borrowers: Vec::new(),
            books: Vec::new(),
        }
    }

    pub fn add_unique_borrower(mut self, br: Borrower) -> Self {
        if !self.borrowers.contains(&br) {
            self.borrowers.push(br);
        }
        self
    }

    pub fn add_unique_book(mut self, bk: Book) -> Self {
        if !self.books.contains(&bk) {
            self.books.push(bk);
        }
        self
    }

    fn brs_len(&self) -> usize {
        self.borrowers.len()
    }

    fn bks_len(&self) -> usize {
        self.books.len()
    }

    pub fn remove_book(mut self, bk: Book) -> Self {
        self.books.retain(|this_bk| this_bk != &bk);
        self
    }

    pub fn find_borrower(self, name: &str) -> (Option<Borrower>, Self) {
        let orig_lib = self.clone();
        let mut brs_into_iter = self.borrowers.into_iter();
        let mbr = brs_into_iter.find(|br| br.get_name() == name);
        (mbr, orig_lib)
    }

    pub fn find_book(self, title: &str) -> (Option<Book>, Self) {
        let orig_lib = self.clone();
        let mut bks_into_iter = self.books.into_iter();
        let mbk = bks_into_iter.find(|bk| bk.get_title() == title);
        (mbk, orig_lib)
    }
}

//fn num_books_out(bks: &[Book], br: &Borrower) -> u8 {
//    let mut count: u8 = 0;
//    for bk in bks {
//        if bk.get_borrower() == Some(br) {
//            count += 1;
//        }
//    }
//    count
//}
//
//fn not_maxed_out(bks: &[Book], br: &Borrower) -> bool {
//    let out = num_books_out(&bks, br);
//    let max = br.get_max_books();
//    out < max
//}
//
//fn book_not_out(bk: &Book) -> bool {
//    bk.get_borrower().is_none()
//}
//
//fn book_out(bk: &Book) -> bool {
//    bk.get_borrower().is_some()
//}
//
//pub fn check_out(
//    brs: Vec<Borrower>,
//    bks: Vec<Book>,
//    name: &str,
//    title: &str,
//) -> (Vec<Book>, Vec<Borrower>) {
//    let orig_bks = bks.clone();
//    let (mbr, brs) = find_borrower(brs, name);
//    let (mbk, bks) = find_book(bks, title);
//    if mbr.is_some()
//        && mbk.is_some()
//        && not_maxed_out(&bks, &mbr.clone().unwrap())
//        && book_not_out(&mbk.clone().unwrap())
//    {
//        let bk = mbk.unwrap();
//        let new_book = bk.clone().set_borrower(mbr);
//        let fewer_bks = remove_book(bks, bk);
//        let new_bks = add_book(fewer_bks, new_book);
//        (new_bks, brs)
//    } else {
//        (orig_bks, brs)
//    }
//}
//
//pub fn check_in(bks: Vec<Book>, title: &str) -> Vec<Book> {
//    let orig_bks = bks.clone();
//    let (mbk, bks) = find_book(bks, title);
//    if mbk.is_some() && book_out(&mbk.clone().unwrap()) {
//        let bk = mbk.unwrap();
//        let new_book = bk.clone().set_borrower(None);
//        let fewer_bks = remove_book(bks, bk);
//        //        let new_bks = add_book(fewer_bks, new_book);
//        //        new_bks
//        add_book(fewer_bks, new_book)
//    } else {
//        orig_bks
//    }
//}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_library() {
        let mut lib = Library::new();
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower1", 1);

        // add borrower
        assert_eq!(lib.brs_len(), 0);
        lib = lib.add_unique_borrower(br1.clone());
        assert_eq!(lib.brs_len(), 1);
        lib = lib.add_unique_borrower(br2.clone());
        assert_eq!(lib.brs_len(), 1);

        // add book
        let bk1 = Book::new("Title1", "Author1", Some(br1));
        let bk2 = Book::new("Title1", "Author1", Some(br2));

        assert_eq!(lib.bks_len(), 0);
        lib = lib.add_unique_book(bk1);
        assert_eq!(lib.bks_len(), 1);
        lib = lib.add_unique_book(bk2.clone());
        assert_eq!(lib.bks_len(), 1);

        // remove book
        let bk3 = Book::new("Title3", "Author3", Some(Borrower::new("Borrower3", 3)));
        let bk4 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));

        assert_eq!(lib.bks_len(), 1);

        lib = lib.remove_book(bk2);
        assert_eq!(lib.bks_len(), 0);

        lib = lib.add_unique_book(bk3);
        assert_eq!(lib.bks_len(), 1);

        lib = lib.remove_book(bk4);
        assert_eq!(lib.bks_len(), 1);

        // find borrower
        let (fnd_br, lib) = lib.find_borrower("Borrower1");
        assert_eq!(fnd_br, Some(Borrower::new("Borrower1", 1)));
        let (fnd_br, lib) = lib.find_borrower("Borrower11");
        assert_eq!(fnd_br, None);

        // find book
        let (fnd_bk, lib) = lib.find_book("Title3");
        assert_eq!(
            fnd_bk,
            Some(Book::new(
                "Title3",
                "Author3",
                Some(Borrower::new("Borrower3", 3)),
            ))
        );
        let (fnd_bk, lib) = lib.find_book("Title11");
        assert_eq!(fnd_bk, None);
    }

    //    #[test]
    //    fn test_num_books_out() {
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let br3 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(br1);
    //        let sbr2 = Some(br2);
    //        let sbr3 = Some(br3);
    //        let bk1 = Book::new("Title1", "Author1", sbr1);
    //        let bk2 = Book::new("Title2", "Author2", sbr2);
    //        let bk3 = Book::new("Title3", "Author3", sbr3);
    //        let bks1: Vec<Book> = Vec::new();
    //        let bks1 = add_book(bks1, bk1);
    //        let bks1 = add_book(bks1, bk2);
    //        let bks1 = add_book(bks1, bk3);
    //        assert_eq!(bks1.len(), 3);
    //
    //        let fnd_num_bks_2 = num_books_out(&bks1, &Borrower::new("Borrower2", 2));
    //        assert_eq!(fnd_num_bks_2, 2);
    //
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let br3 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(br1);
    //        let sbr2 = Some(br2);
    //        let sbr3 = Some(br3);
    //        let bk1 = Book::new("Title1", "Author1", sbr1);
    //        let bk2 = Book::new("Title2", "Author2", sbr2);
    //        let bk3 = Book::new("Title3", "Author3", sbr3);
    //        let bks1: Vec<Book> = Vec::new();
    //        let bks1 = add_book(bks1, bk1);
    //        let bks1 = add_book(bks1, bk2);
    //        let bks1 = add_book(bks1, bk3);
    //
    //        let fnd_num_bks_1 = num_books_out(&bks1, &Borrower::new("Borrower1", 1));
    //        assert_eq!(fnd_num_bks_1, 1);
    //
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let br3 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(br1);
    //        let sbr2 = Some(br2);
    //        let sbr3 = Some(br3);
    //        let bk1 = Book::new("Title1", "Author1", sbr1);
    //        let bk2 = Book::new("Title2", "Author2", sbr2);
    //        let bk3 = Book::new("Title3", "Author3", sbr3);
    //        let bks1: Vec<Book> = Vec::new();
    //        let bks1 = add_book(bks1, bk1);
    //        let bks1 = add_book(bks1, bk2);
    //        let bks1 = add_book(bks1, bk3);
    //
    //        let none_fnd_bks = num_books_out(&bks1, &Borrower::new("Borrower22", 2));
    //        assert_eq!(none_fnd_bks, 0);
    //    }
    //
    //    #[test]
    //    fn test_not_maxed_out() {
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(br1);
    //        let sbr2 = Some(br2);
    //        let bk1 = Book::new("Title1", "Author1", sbr1);
    //        let bk2 = Book::new("Title1", "Author1", sbr2);
    //        let bks1: Vec<Book> = Vec::new();
    //        let bks1 = add_book(bks1, bk1);
    //        let bks1 = add_book(bks1, bk2);
    //
    //        let not_maxed_br1 = not_maxed_out(&bks1, &Borrower::new("Borrower1", 1));
    //        assert_eq!(not_maxed_br1, false);
    //
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(br1);
    //        let sbr2 = Some(br2);
    //        let mut bk1 = Book::new("Title1", "Author1", sbr1);
    //        let mut bk2 = Book::new("Title1", "Author1", sbr2);
    //        let bks1: Vec<Book> = Vec::new();
    //        let bks1 = add_book(bks1, bk1);
    //        let bks1 = add_book(bks1, bk2);
    //
    //        let not_maxed_br2 = not_maxed_out(&bks1, &Borrower::new("Borrower2", 2));
    //        assert_eq!(not_maxed_br2, true);
    //    }
    //
    //    #[test]
    //    fn test_check_out() {
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let brs1: Vec<Borrower> = Vec::new();
    //        let brs1 = add_borrower(brs1, br1.clone());
    //        let brs1 = add_borrower(brs1, br2.clone());
    //
    //        let sbr1 = Some(br1);
    //        let sbr2 = Some(br2);
    //        let bk1 = Book::new("Title1", "Author1", sbr1);
    //        let bk2 = Book::new("Title2", "Author2", None);
    //        let bk3 = Book::new("Title2", "Author2", sbr2);
    //
    //        let bks1: Vec<Book> = Vec::new();
    //        let bks1 = add_book(bks1, bk1.clone());
    //        let bks1 = add_book(bks1, bk2.clone());
    //        let bks2: Vec<Book> = Vec::new();
    //        let bks2 = add_book(bks2, bk1);
    //        let bks2 = add_book(bks2, bk3);
    //
    //        // check-out-pass-test
    //        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "Borrower2", "Title2");
    //        assert_eq!(ret_bks, bks2);
    //
    //        // check-out-fail-checked-out-test
    //        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "Borrower2", "Title1");
    //        assert_eq!(ret_bks, bks1);
    //
    //        // check-out-fail-bad-book-test
    //        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "Borrower2", "NoTitle");
    //        assert_eq!(ret_bks, bks1);
    //
    //        // check-out-fail-bad-borrower-test
    //        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "NoName", "Title2");
    //        assert_eq!(ret_bks, bks1);
    //
    //        // check-out-fail-over-limit-test
    //        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "Borrower1", "Title2");
    //        assert_eq!(ret_bks, bks1);
}
//}
