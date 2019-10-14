use crate::book::Book;
use crate::borrower::Borrower;
use std::ops::Deref;

pub fn add_item<'a, T: PartialEq>(mut xs: Vec<&'a T>, x: &'a T) -> Vec<&'a T> {
    if xs.contains(&x) {
        xs
    } else {
        xs.push(x);
        xs
    }
}

pub fn remove_book<'a>(mut bks: Vec<&'a Book<'a>>, bk: &Book) -> Vec<&'a Book<'a>> {
    bks.retain(|this_bk| this_bk != &bk);
    bks
}

//fn find_item<'a, T, F>(target: &str, coll: Vec<&'a T>, f: F) -> Option<&'a T> where
//    F: Fn(&T) -> &str {
//    let mut iterator = coll.into_iter();
//    let maybe_match = iterator.find(|a| f(a) == target);
//    maybe_match
//}

fn find_borrower<'a>(name: &str, brs: &'a Vec<&'a Borrower>) -> Option<&'a &'a Borrower> {
    let mut iterator = brs.into_iter();
    let maybe_borrower = iterator.find(|br| br.name == name.to_owned());
    maybe_borrower
}

//fn find_book<'a>(title: &str, bks: &'a Vec<&'a mut Book<'a>>) -> Option<&'a &'a mut Book<'a>> {
//    let mut iterator = bks.into_iter();
//    let maybe_mut_book = iterator.find(|bk| bk.title == title.to_owned());
//    maybe_mut_book
//}

fn find_book<'a>(title: &str, bks: &'a Vec<&'a mut Book<'a>>) -> (usize, Option<&'a &'a mut Book<'a>>) {
    let mut result: (usize, Option<&'a &'a mut Book<'a>>) = (0, None);
    for (i, bk) in bks.into_iter().enumerate() {
        result = if bk.title == title.to_owned() {
            (i, Some(bk))
        } else {
            result
        }
    }
    result
}

fn find_num(target: i32, nums: &Vec<i32>) -> (usize, i32) {
    let mut result = (0, 0);
    for (i, item) in nums.into_iter().enumerate() {
        result = if *item == target {
            (i, *item)
        } else {
            result
        }
    }
    result
}

fn find_name<'a>(target: &str, names: &Vec<&'a str>) -> (usize, &'a str) {
    let mut result = (0, "none");
    for (i, item) in names.into_iter().enumerate() {
        result = if *item == target {
            (i, *item)
        } else {
            result
        }
    }
    result
}

fn num_books_out(bks: &Vec<Book>, br: &Borrower) -> usize {
    let mut iterator = bks.into_iter();
    let num_books_out = iterator.filter(|bk| bk.borrower == Some(br)).count();
    num_books_out
}

fn not_maxed_out(bks: &Vec<Book>, br: &Borrower) -> bool {
    let out = num_books_out(bks, br);
    let max = br.max_books;
    out < max as usize
}

fn book_not_out(bk: &Book) -> bool {
    bk.borrower.is_none()
}

fn book_out(bk: &Book) -> bool {
    bk.borrower.is_some()
}


//pub fn check_out<'a>(name: &str, title: &str, brs: &Vec<&Borrower>, bks: Vec<Book<'a>>) -> Vec<Book<'a>> {
//    let mbr = find_borrower(name, brs);
//    let mbk = find_book(title, bks.clone());
//    if mbr.is_some()
//        && mbk.is_some()
//        && not_maxed_out(&bks, mbr.unwrap())
//        && book_not_out(&mbk.unwrap()) {
//        let bk = mbk.unwrap();
//        bk.clone().set_borrower(*mbr);
////        let mut fewer_books = remove_book(bks.clone(), bk);
////        let vv = add_item(fewer_books, &new_book);
////        bks.clone()
////    } else {
////        bks
////    }
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
    fn test_add_item() {
        let br1 = &Borrower::new("Borrower1", 1);
        let br2 = &Borrower::new("Borrower2", 2);
        let brs1: Vec<&Borrower> = vec![br1];
        let brs2: Vec<&Borrower> = vec![br1, br2];
        assert_eq!(brs1.len(), 1);
        assert_eq!(brs2.len(), 2);
        assert_eq!(add_item(brs1.clone(), br2), brs2);
        assert_eq!(add_item(brs1.clone(), br1), brs1);

        let bk1 = &Book::new("Title1", "Author1", Some(&br1));
        let bk2 = &Book::new("Title1", "Author1", Some(&br2));
        let bks1: Vec<&Book> = vec![bk1];
        let bks2: Vec<&Book> = vec![bk1, bk2];
        assert_eq!(bks1.len(), 1);
        assert_eq!(bks2.len(), 2);
        assert_eq!(add_item(bks1.clone(), bk2), bks2);
        assert_eq!(add_item(bks1.clone(), bk1), bks1);
    }

    #[test]
    fn test_remove_book() {
        let br1 = &Borrower::new("Borrower1", 1);
        let br2 = &Borrower::new("Borrower2", 2);
        let bk1 = &Book::new("Title1", "Author1", Some(&br1));
        let bk2 = &Book::new("Title1", "Author1", Some(&br2));
        let bks1: Vec<&Book> = vec![bk1, bk2];
        let bks2: Vec<&Book> = vec![bk1];
        assert_eq!(bks1.len(), 2);
        assert_eq!(bks2.len(), 1);
        assert_eq!(remove_book(bks1, bk2), bks2);
        assert_eq!(remove_book(bks2.clone(), bk2), bks2)
    }

    #[test]
    fn test_find_borrower() {
        let br1 = Borrower { name: "Borrower1".to_owned(), max_books: 1 };
        let br2 = Borrower { name: "Borrower2".to_owned(), max_books: 2 };
        let brs: &Vec<&Borrower> = &vec![&br1, &br2];
        let actual_ptr = find_borrower("Borrower1", brs);
        assert_eq!(actual_ptr, Some(&&Borrower { name: "Borrower1".to_owned(), max_books: 1 }));
        let actual_ptr_2 = find_borrower("Borrower11", brs);
        assert_eq!(actual_ptr_2, None)
    }

//    #[test]
//    fn test_find_book() {
//        let br1 = Borrower::new("Borrower1", 1);
//        let br2 = Borrower::new("Borrower2", 2);
//        let mut bk1 = Book::new("Title1", "Author1", Some(&br1));
//        let mut bk2 = Book::new("Title1", "Author1", Some(&br2));
//        let bks: &Vec<&mut Book> = &vec![&mut bk1, &mut bk2];
//        let actual_ptr = find_book("Title1", bks);
//        assert_eq!(actual_ptr, Some(&&mut Book::new("Title1", "Author1", Some(&br1))));
//        let actual_ptr_2 = find_book("Title11", bks);
//        assert_eq!(actual_ptr_2, None)
//    }

    #[test]
    fn test_find_num() {
        let foo = vec![1, 35, 64, 36, 26];
        let r = find_num(26, &foo);
        assert_eq!(r, (4, 26));
        let r2 = find_num(1, &foo);
        assert_eq!(r2, (0, 1));
        let r3 = find_num(11, &foo);
        assert_eq!(r3, (0, 0))
    }

    #[test]
    fn test_find_name() {
        let foo = vec!["one", "two", "three"];
        let r = find_name("one", &foo);
        assert_eq!(r, (0, "one"));
        let r2 = find_name("three", &foo);
        assert_eq!(r2, (2, "three"));
        let r3 = find_name("nine", &foo);
        assert_eq!(r3, (0, "none"))
    }

    #[test]
    fn test_num_books_out() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let sbr1 = Some(&br1);
        let sbr2 = Some(&br2);
        let bk1 = Book::new("Title1", "Author1", sbr1);
        let bk2 = Book::new("Title2", "Author2", sbr2);
        let bk3 = Book::new("Title3", "Author3", sbr2);
        let bks: Vec<Book> = vec![bk1, bk2, bk3];
        let books_out_br1 = num_books_out(&bks, &Borrower::new("Borrower1", 1));
        assert_eq!(books_out_br1, 1);
        let books_out_br2 = num_books_out(&bks, &Borrower::new("Borrower2", 2));
        assert_eq!(books_out_br2, 2);
        let books_out_br0 = num_books_out(&bks, &Borrower::new("Borrower0", 0));
        assert_eq!(books_out_br0, 0)
    }

    #[test]
    fn test_not_maxed_out() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let sbr1 = Some(&br1);
        let sbr2 = Some(&br2);
        let bk1 = Book::new("Title1", "Author1", sbr1);
        let bk2 = Book::new("Title2", "Author2", sbr2);
        let bks: Vec<Book> = vec![bk1, bk2];
        let br1_is_not_maxed = not_maxed_out(&bks, &Borrower::new("Borrower1", 1));
        assert_eq!(br1_is_not_maxed, false);
        let br2_is_not_maxed = not_maxed_out(&bks, &Borrower::new("Borrower2", 2));
        assert_eq!(br2_is_not_maxed, true);
    }

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
}
