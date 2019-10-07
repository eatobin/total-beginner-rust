//use crate::book::Book;
use crate::borrower::Borrower;
//
//#[derive(Debug, PartialEq, Clone)]
//pub struct Library {
//    borrowers: Vec<Borrower>,
//    books: Vec<Book>,
//}
//

pub fn add_borrower(mut brs: Vec<Borrower>, br: Borrower) -> Vec<Borrower> {
    if brs.contains(&br) {
        brs
    } else {
        brs.push(br);
        brs
    }
}

//impl Library {
//    pub fn new() -> Library {
//        Library {
//            borrowers: Vec::new(),
//            books: Vec::new(),
//        }
//    }
//
//    pub fn add_unique_borrower(mut self, br: Borrower) -> Self {
//        if !self.borrowers.contains(&br) {
//            self.borrowers.push(br);
//        }
//        self
//    }
//
//    pub fn add_unique_book(mut self, bk: Book) -> Self {
//        if !self.books.contains(&bk) {
//            self.books.push(bk);
//        }
//        self
//    }
//
//    fn brs_len(&self) -> usize {
//        self.borrowers.len()
//    }
//
//    fn bks_len(&self) -> usize {
//        self.books.len()
//    }
//
//    pub fn remove_book(mut self, bk: Book) -> Self {
//        self.books.retain(|this_bk| this_bk != &bk);
//        self
//    }
//
//    pub fn find_borrower(self, name: &str) -> (Option<Borrower>, Self) {
//        let orig_lib = self.clone();
//        let mut brs_into_iter = self.borrowers.into_iter();
//        let mbr = brs_into_iter.find(|br| br.get_name() == name);
//        (mbr, orig_lib)
//    }
//
//    pub fn find_book(self, title: &str) -> (Option<Book>, Self) {
//        let orig_lib = self.clone();
//        let mut bks_into_iter = self.books.into_iter();
//        let mbk = bks_into_iter.find(|bk| bk.get_title() == title);
//        (mbk, orig_lib)
//    }
//
//    fn num_books_out(&self, br: &Borrower) -> u8 {
//        let mut count: u8 = 0;
//        for bk in &self.books {
//            if bk.get_borrower() == Some(br) {
//                count += 1;
//            }
//        }
//        count
//    }
//
//    fn not_maxed_out(&self, br: &Borrower) -> bool {
//        let out = self.num_books_out(br);
//        let max = br.get_max_books();
//        out < max
//    }
//
//    fn book_not_out(bk: &Book) -> bool {
//        bk.get_borrower().is_none()
//    }
//
//    fn book_out(bk: &Book) -> bool {
//        bk.get_borrower().is_some()
//    }
//
//    pub fn check_out(mut self, name: &str, title: &str) -> Self {
//        let (mbr, _lib) = self.clone().find_borrower(name);
//        let (mbk, _lib) = self.clone().find_book(title);
//        if mbr.is_some()
//            && mbk.is_some()
//            && self.not_maxed_out(&mbr.clone().unwrap())
//            && Library::book_not_out(&mbk.clone().unwrap())
//        {
//            let bk = mbk.unwrap();
//            let new_book = bk.clone().set_borrower(mbr);
//            self = self.remove_book(bk);
//            self.add_unique_book(new_book)
//        } else {
//            self
//        }
//    }
//
//    pub fn check_in(mut self, title: &str) -> Self {
//        let (mbk, _lib) = self.clone().find_book(title);
//        if mbk.is_some() && Library::book_out(&mbk.clone().unwrap()) {
//            let bk = mbk.unwrap();
//            let new_book = bk.clone().set_borrower(None);
//            self = self.remove_book(bk);
//            self.add_unique_book(new_book)
//        } else {
//            self
//        }
//    }
//}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_library() {
//        let mut lib = Library::new();
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let brs1: Vec<Borrower> = vec![br1.clone()];
        let brs2: Vec<Borrower> = vec![br1.clone(), br2.clone()];

        // add borrower
        assert_eq!(brs1.len(), 1);
        assert_eq!(brs2.len(), 2);
        assert_eq!(add_borrower(brs1.clone(), br2), brs2)

//        lib = lib.add_unique_borrower(br1.clone());
//        assert_eq!(lib.brs_len(), 1);
//        lib = lib.add_unique_borrower(br2.clone());
//        assert_eq!(lib.brs_len(), 1);

//        // add book
//        let bk1 = Book::new("Title1", "Author1", Some(br1));
//        let bk2 = Book::new("Title1", "Author1", Some(br2));
//
//        assert_eq!(lib.bks_len(), 0);
//        lib = lib.add_unique_book(bk1);
//        assert_eq!(lib.bks_len(), 1);
//        lib = lib.add_unique_book(bk2.clone());
//        assert_eq!(lib.bks_len(), 1);
//
//        // remove book
//        let bk3 = Book::new("Title3", "Author3", Some(Borrower::new("Borrower3", 3)));
//        let bk4 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
//
//        assert_eq!(lib.bks_len(), 1);
//
//        lib = lib.remove_book(bk2);
//        assert_eq!(lib.bks_len(), 0);
//
//        lib = lib.add_unique_book(bk3);
//        assert_eq!(lib.bks_len(), 1);
//
//        lib = lib.remove_book(bk4);
//        assert_eq!(lib.bks_len(), 1);
//
//        // find borrower
//        let (fnd_br, lib) = lib.find_borrower("Borrower1");
//        assert_eq!(fnd_br, Some(Borrower::new("Borrower1", 1)));
//        let (fnd_br, lib) = lib.find_borrower("Borrower11");
//        assert_eq!(fnd_br, None);
//
//        // find book
//        let (fnd_bk, lib) = lib.find_book("Title3");
//        assert_eq!(
//            fnd_bk,
//            Some(Book::new(
//                "Title3",
//                "Author3",
//                Some(Borrower::new("Borrower3", 3)),
//            ))
//        );
//        let (fnd_bk, lib) = lib.find_book("Title11");
//        assert_eq!(fnd_bk, None);
//
//        // num books out
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower2", 2);
//        let br3 = Borrower::new("Borrower2", 2);
//        let sbr1 = Some(br1.clone());
//        let sbr2 = Some(br2.clone());
//        let sbr3 = Some(br3.clone());
//        let bk1 = Book::new("Title1", "Author1", sbr1);
//        let bk2 = Book::new("Title2", "Author2", sbr2);
//        let bk3 = Book::new("Title3", "Author3", sbr3);
//        let mut lib = Library::new();
//        lib = lib.add_unique_book(bk1.clone());
//        lib = lib.add_unique_book(bk2);
//        lib = lib.add_unique_book(bk3);
//        assert_eq!(lib.bks_len(), 3);
//
//        let fnd_num_bks_2 = Library::num_books_out(&lib, &Borrower::new("Borrower2", 2));
//        assert_eq!(fnd_num_bks_2, 2);
//
//        let fnd_num_bks_1 = Library::num_books_out(&lib, &Borrower::new("Borrower1", 1));
//        assert_eq!(fnd_num_bks_1, 1);
//
//        let none_fnd_bks = Library::num_books_out(&lib, &Borrower::new("Borrower22", 2));
//        assert_eq!(none_fnd_bks, 0);
//
//        // not_maxed_out
//        let not_maxed_br1 = lib.not_maxed_out(&Borrower::new("Borrower1", 1));
//        assert_eq!(not_maxed_br1, false);
//
//        let br2 = br2.set_max_books(3);
//        let not_maxed_br2 = lib.not_maxed_out(&Borrower::new("Borrower2", 3));
//        assert_eq!(not_maxed_br2, true);
//
//        // check_out
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower2", 2);
//
//        let sbr1 = Some(br1.clone());
//        let sbr2 = Some(br2.clone());
//        let bk1 = Book::new("Title1", "Author1", sbr1);
//        let bk2 = Book::new("Title2", "Author2", None);
//        let bk3 = Book::new("Title2", "Author2", sbr2);
//
//        let lib1 = Library::new();
//        let lib1 = lib1.add_unique_borrower(br1.clone());
//        let lib1 = lib1.add_unique_borrower(br2.clone());
//        let lib1 = lib1.add_unique_book(bk1.clone());
//        let lib1 = lib1.add_unique_book(bk2);
//        let lib2 = Library::new();
//        let lib2 = lib2.add_unique_borrower(br1);
//        let lib2 = lib2.add_unique_borrower(br2);
//        let lib2 = lib2.add_unique_book(bk1);
//        let lib2 = lib2.add_unique_book(bk3);
//
//        // check-out-pass-test
//        let ret_lib = lib1.clone().check_out("Borrower2", "Title2");
//        assert_eq!(ret_lib, lib2);
//
//        // check-out-fail-checked-out-test
//        let ret_lib = lib1.clone().check_out("Borrower2", "Title1");
//        assert_eq!(ret_lib, lib1);
//
//        // check-out-fail-bad-book-test
//        let ret_lib = lib1.clone().check_out("Borrower2", "NoTitle");
//        assert_eq!(ret_lib, lib1);
//
//        // check-out-fail-bad-borrower-test
//        let ret_lib = lib1.clone().check_out("NoName", "Title2");
//        assert_eq!(ret_lib, lib1);
//
//        // check-out-fail-over-limit-test
//        let ret_lib = lib1.clone().check_out("Borrower1", "Title2");
//        assert_eq!(ret_lib, lib1);
//
//        // check-in-pass-test
//        let ret_lib = lib1.clone().check_in("Title2");
//        assert_eq!(ret_lib, lib1);
//
//        // check-in-fail-not-checked-out-test
//        let ret_lib = lib1.clone().check_in("Title2");
//        assert_eq!(ret_lib, lib1);
//
//        // check-in-fail-bad-book-test
//        let ret_lib = lib1.clone().check_in("NoTitle");
//        assert_eq!(ret_lib, lib1);
    }
}
