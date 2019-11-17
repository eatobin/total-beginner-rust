use crate::book::Book;
use crate::borrower::Borrower;
use std::fs::read;

pub fn add_borrower<'a>(mut brs: Vec<&'a Borrower>, br: &'a Borrower) -> Vec<&'a Borrower> {
    if brs.contains(&br) {
        brs
    } else {
        brs.push(br);
        brs
    }
}

pub fn add_book<'a>(mut bks: Vec<Book<'a>>, bk: Book<'a>) -> Vec<Book<'a>> {
    if bks.contains(&bk) {
        bks
    } else {
        bks.push(bk);
        bks
    }
}

//pub fn remove_book<'a>(mut bks: Vec<Book<'a>>, bk: Book) -> Vec<Book<'a>> {
//    bks.retain(|this_bk| this_bk != &bk);
//    bks
//}

fn find_borrower<'a>(n: &str, brs: Vec<&'a Borrower>) -> Option<&'a Borrower> {
    let mut iterator = brs.into_iter();
    let maybe_match = iterator.find(|br| br.name == n.to_string());
    maybe_match
}

fn find_book<'a>(t: &str, bks: Vec<Book<'a>>) -> Option<(usize, Book<'a>)> {
    let mut iterator = bks.into_iter().enumerate();
    let maybe_match = iterator.find(|(i, bk)| bk.title == t.to_string());
    maybe_match
}

//fn find_item<'a, T, F>(target: &str, coll: Vec<&'a T>, f: F) -> Option<&'a T> where
//    F: Fn(&T) -> &str {
//    let mut iterator = coll.into_iter();
//    let maybe_match = iterator.find(|a| f(a) == target);
//    maybe_match
//}

//fn num_books_out(bks: Vec<&Book>, br: &Borrower) -> usize {
//    let mut iterator = bks.into_iter();
//    let num_books_out = iterator.filter(|bk| bk.get_borrower() == Some(br)).count();
//    num_books_out
//}

//fn not_maxed_out(bks: Vec<&Book>, br: &Borrower) -> bool {
//    let out = num_books_out(bks, br);
//    let max = br.get_max_books();
//    out < max as usize
//}
//
//fn book_not_out(bk: &Book) -> bool {
//    bk.get_borrower().is_none()
//}
//
//fn book_out(bk: &Book) -> bool {
//    bk.get_borrower().is_some()
//}


//pub fn check_out<'a>(name: &str, title: &str, brs: &Vec<Borrower>, mut bks: Vec<Book<'a>>) -> Vec<Book<'a>> {
//    let mbr = find_item(name, brs, Borrower::get_name);
//    let mbk = find_item(title, bks.clone(), Book::get_title);
//    if mbr.is_some()
//        && mbk.is_some()
//        && not_maxed_out(bks.clone(), mbr.unwrap())
//        && book_not_out(mbk.unwrap()) {
//        let bk = mbk.unwrap();
//        let new_book = bk.clone().set_borrower(mbr);
//        let mut fewer_books = remove_book(bks.clone(), bk);
//        let vv = add_item(fewer_books, &new_book);
//        bks.clone()
//    } else {
//        bks
//    }
//}


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
    fn test_add_borrower() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let mut brs1: Vec<&Borrower> = vec![&br1];
        let mut brs2: Vec<&Borrower> = vec![&br1, &br2];
        assert_eq!(brs1.len(), 1);
        assert_eq!(brs2.len(), 2);
        assert_eq!(add_borrower(brs1.clone(), &br2), brs2);
        assert_eq!(add_borrower(brs1.clone(), &br1), brs1);
    }

    #[test]
    fn test_add_book() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let bk1 = Book::new("Title1", "Author1", Some(&br1));
        let bk2 = Book::new("Title1", "Author1", Some(&br2));
        let bks1: Vec<Book> = vec![bk1.clone()];
        let bks2: Vec<Book> = vec![bk1.clone(), bk2.clone()];
        assert_eq!(bks1.len(), 1);
        assert_eq!(bks2.len(), 2);
        assert_eq!(add_book(bks1.clone(), bk2), bks2);
        assert_eq!(add_book(bks1.clone(), bk1), bks1);
    }
}

//    #[test]
//    fn test_remove_book() {
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower2", 2);
//        let bk1 = Book::new("Title1", "Author1", Some(&br1));
//        let bk2 = Book::new("Title1", "Author1", Some(&br2));
//        let bks1: Vec<Book> = vec![bk1.clone(), bk2.clone()];
//        let bks2: Vec<Book> = vec![bk1];
//        assert_eq!(bks1.len(), 2);
//        assert_eq!(bks2.len(), 1);
//        assert_eq!(remove_book(bks1, bk2.clone()), bks2);
//        assert_eq!(remove_book(bks2.clone(), bk2), bks2)
//    }

//    #[test]
//    fn test_find_borrower() {
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower2", 2);
//        let brs: Vec<&Borrower> = vec![&br1, &br2];
//        let actual_ptr = find_item("Borrower1", brs.clone(), Borrower::get_name);
//        assert_eq!(actual_ptr, Some(Borrower::new("Borrower1", 1)).as_ref());
//        let actual_ptr_2 = find_item("Borrower11", brs, Borrower::get_name);
//        assert_eq!(actual_ptr_2, None)
//    }

//    #[test]
//    fn test_find_book() {
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower2", 2);
//        let bk1 = Book::new("Title1", "Author1", Some(&br1));
//        let bk2 = Book::new("Title1", "Author1", Some(&br2));
//        let bks: Vec<&Book> = vec![&bk1, &bk2];
//        let actual_ptr = find_item("Title1", bks.clone(), Book::get_title);
//        assert_eq!(actual_ptr, Some(Book::new("Title1", "Author1", Some(&br1))).as_ref());
//        let actual_ptr_2 = find_item("Title11", bks, Book::get_title);
//        assert_eq!(actual_ptr_2, None)
//    }
//
//    #[test]
//    fn test_num_books_out() {
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower2", 2);
//        let sbr1 = Some(&br1);
//        let sbr2 = Some(&br2);
//        let bk1 = Book::new("Title1", "Author1", sbr1);
//        let bk2 = Book::new("Title2", "Author2", sbr2);
//        let bk3 = Book::new("Title3", "Author3", sbr2);
//        let bks: Vec<&Book> = vec![&bk1, &bk2, &bk3];
//        let books_out_br1 = num_books_out(bks.clone(), &Borrower::new("Borrower1", 1));
//        assert_eq!(books_out_br1, 1);
//        let books_out_br2 = num_books_out(bks.clone(), &Borrower::new("Borrower2", 2));
//        assert_eq!(books_out_br2, 2);
//        let books_out_br0 = num_books_out(bks, &Borrower::new("Borrower0", 0));
//        assert_eq!(books_out_br0, 0)
//    }
//
//    #[test]
//    fn test_not_maxed_out() {
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower2", 2);
//        let sbr1 = Some(&br1);
//        let sbr2 = Some(&br2);
//        let bk1 = Book::new("Title1", "Author1", sbr1);
//        let bk2 = Book::new("Title2", "Author2", sbr2);
//        let bks: Vec<&Book> = vec![&bk1, &bk2];
//        let br1_is_not_maxed = not_maxed_out(bks.clone(), &Borrower::new("Borrower1", 1));
//        assert_eq!(br1_is_not_maxed, false);
//        let br2_is_not_maxed = not_maxed_out(bks.clone(), &Borrower::new("Borrower2", 2));
//        assert_eq!(br2_is_not_maxed, true);
//    }

//    #[test]
//    fn test_check_out() {
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower2", 2);
//        let brs: Vec<&Borrower> = vec![&br1, &br2];
//        let sbr1 = Some(&br1);
//        let sbr2 = Some(&br2);
//        let bk1 = Book::new("Title1", "Author1", sbr1);
//        let bk2 = Book::new("Title2", "Author2", None);
//        let bk3 = Book::new("Title2", "Author2", sbr2);
//        let bks1: Vec<&Book> = vec![&bk1, &bk2];
//        let bks2: Vec<&Book> = vec![&bk1, &bk3];
//        let pass = check_out("Borrower2", "Title2", brs, bks1);
//        assert_eq!(pass, bks2)
//    }


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
//}
