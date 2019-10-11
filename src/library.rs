use crate::book::Book;
use crate::borrower::Borrower;

pub fn add_item<'a, T: PartialEq>(mut xs: Vec<&'a T>, x: &'a T) -> Vec<&'a T> {
    if xs.contains(&x) {
        xs
    } else {
        xs.push(x);
        xs
    }
}

pub fn remove_book<'a>(mut bks: Vec<&'a Book>, bk: &Book) -> Vec<&'a Book> {
    bks.retain(|this_bk| this_bk != &bk);
    bks
}

pub fn find_borrower<'br>(name: &str, brs: Vec<&'br Borrower>) -> Option<&'br Borrower> {
    let mut brs_into_iter = brs.into_iter();
    brs_into_iter.find(|br| br.get_name() == name)
}

//pub fn find_mutable_book<'bk4>(title: &str, bks: Vec<&'bk4 Book>) -> Option<&'bk4 Book<'bk4>> {
//    let mut bks_into_iter = bks.into_iter();
//    bks_into_iter.find(|br| br.get_title() == title)
//}

// pub fn find_item<'x, T: PartialEq>(target: &str, coll: Vec<&'x T>, f: &dyn Fn(&T) -> &'x str) -> Option<&'x T> {
//     let mut coll_into_iter = coll.into_iter();
//     coll_into_iter.find(|i| f(i) == target)
// }

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
    fn test_add_item() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let brs1: Vec<&Borrower> = vec![&br1];
        let brs2: Vec<&Borrower> = vec![&br1, &br2];
        assert_eq!(brs1.len(), 1);
        assert_eq!(brs2.len(), 2);
        assert_eq!(add_item(brs1.clone(), &br2), brs2);
        assert_eq!(add_item(brs1.clone(), &br1), brs1);

        let bk1 = Book::new("Title1", "Author1", Some(br1));
        let bk2 = Book::new("Title1", "Author1", Some(br2));
        let bks1: Vec<&Book> = vec![&bk1];
        let bks2: Vec<&Book> = vec![&bk1, &bk2];
        assert_eq!(bks1.len(), 1);
        assert_eq!(bks2.len(), 2);
        assert_eq!(add_item(bks1.clone(), &bk2), bks2);
        assert_eq!(add_item(bks1.clone(), &bk1), bks1);
    }

    #[test]
    fn test_remove_book() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let bk1 = Book::new("Title1", "Author1", Some(br1));
        let bk2 = Book::new("Title1", "Author1", Some(br2));
        let bks1: Vec<&Book> = vec![&bk1, &bk2];
        let bks2: Vec<&Book> = vec![&bk1];
        assert_eq!(bks1.len(), 2);
        assert_eq!(bks2.len(), 1);
        assert_eq!(remove_book(bks1, &bk2), bks2);
        assert_eq!(remove_book(bks2.clone(), &bk2), bks2);

        //        lib = lib.remove_book(bk2);
        //        assert_eq!(lib.bks_len(), 0);
        //
        //        lib = lib.add_unique_book(bk3);
        //        assert_eq!(lib.bks_len(), 1);
        //
        //        lib = lib.remove_book(bk4);
        //        assert_eq!(lib.bks_len(), 1);
    }

    #[test]
    fn test_find_borrower() {
        let br1 = &Borrower::new("Borrower1", 1);
        let br2 = &Borrower::new("Borrower2", 2);
        let brs1: Vec<&Borrower> = vec![br1];
        let brs2: Vec<&Borrower> = vec![br1, &br2];
        let actual_ptr = find_borrower("Borrower1", brs1.clone());
        assert_eq!(actual_ptr, Some(Borrower::new("Borrower1", 1)).as_ref());
        let actual2 = find_borrower("Borrower11", brs1.clone());
        assert_eq!(actual2, None);
    }

    // #[test]
    // fn test_find_item() {
    //     let br1 = &Borrower::new("Borrower1", 1);
    //     let br2 = &Borrower::new("Borrower2", 2);
    //     let brs1: Vec<&Borrower> = vec![br1];
    //     let brs2: Vec<&Borrower> = vec![br1, &br2];
    //     let actual_ptr = find_item("Borrower1", brs1.clone(), Borrower::get_name);
    //     assert_eq!(actual_ptr, Some(Borrower::new("Borrower1", 1)).as_ref());
    //     let actual2 = find_borrower("Borrower11", brs1.clone());
    //     assert_eq!(actual2, None);
    // }
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
