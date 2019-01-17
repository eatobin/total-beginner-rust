use crate::book::Book;
use crate::borrower::Borrower;

pub fn add_item<T>(mut xs: Vec<T>, x: T) -> Vec<T>
where
    T: PartialEq,
{
    if !xs.contains(&x) {
        xs.push(x);
    }
    xs
}

pub fn remove_book(mut bks: Vec<Book>, bk: Book) -> Vec<Book> {
    bks.retain(|this| this != &bk);
    bks
}

pub fn find_borrower<'a>(brs: &'a Vec<Borrower>, name: &str) -> Option<&'a Borrower> {
    brs.iter().find(|&br| Borrower::get_name(br) == name)
}

pub fn find_book(bks: Vec<Book>, title: &str) -> Option<Book> {
    bks.into_iter().find(|bk| Book::get_title(bk) == title)
}

//#[derive(Debug)]
//pub struct Library {
//    borrowers: Vec<Borrower>,
//    books: Vec<Book>,
//}
//
//impl Library {
//    pub fn new() -> Self {
//        Library {
//            borrowers: Vec::new(),
//            books: Vec::new(),
//        }
//    }
//
//    pub fn get_borrowers(&self) -> &Vec<Borrower> {
//        &(self.borrowers)
//    }
//
//    pub fn get_books(&self) -> &Vec<Book> {
//        &(self.books)
//    }
//
//
//
//    pub fn add_book(&mut self, bk: Book) {
//        if !self.books.contains(&bk) {
//            self.books.push(bk);
//        }
//    }
//
//    pub fn find_borrower(&self, name: &str) -> Option<&Borrower> {
//        self.borrowers
//            .iter()
//            .find(|br| Borrower::get_name(br) == name)
//    }
//
//    pub fn find_book(&self, title: &str) -> Option<&Book> {
//        self.books
//            .iter()
//            .find(|bk| Book::get_title(bk) == title)
//    }
//
//    fn num_books_out(&self, br: &Borrower) -> u8 {
//        let mut count: u8 = 0;
//        for nxt_bk in &(self.books) {
//            if nxt_bk.get_borrower().as_ref() == Some(br) {
//                count += 1;
//            }
//        }
//        count
//    }
//
//    fn not_maxed_out(&self, br: &Borrower) -> bool {
//        let out = Library::num_books_out(&self, br);
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
//        pub fn check_in(self, name: &str, title: &str) -> Library {
//            let lib = self;
//            let mbr = lib.find_borrower(name);
//            let mbk = lib.find_book(title);
////            let mbr = Library::find_borrower(&self, name);
////            let mbk = Library::find_book(&self, title);
////            if mbk.is_some() && mbr.is_some() && Library::not_maxed_out(self, mbr.unwrap()) && Library::book_not_out(mbk.unwrap()) { }
//            if mbr.is_some() && mbk.is_some() && lib.not_maxed_out(mbr.unwrap()) && Library::book_not_out(mbk.unwrap()) {
//                let new_book = Book{borrower: mbr, ..*mbk});
//                lib
//            }else { lib }
//        }
//}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_borrower_or_book() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower1", 1);
        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let bk2 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let mut brs: Vec<Borrower> = Vec::new();
        let mut bks: Vec<Book> = Vec::new();

        assert_eq!(brs.len(), 0);
        let brs = add_item(brs, br1);
        assert_eq!(brs.len(), 1);
        let brs = add_item(brs, br2);
        assert_eq!(brs.len(), 1);

        assert_eq!(bks.len(), 0);
        let bks = add_item(bks, bk1);
        assert_eq!(bks.len(), 1);
        let bks = add_item(bks, bk2);
        assert_eq!(bks.len(), 1);
    }

    #[test]
    fn test_remove_book() {
        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let bk2 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let bk3 = Book::new("Title3", "Author3", Some(Borrower::new("Borrower3", 3)));
        let bk4 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let mut bks: Vec<Book> = Vec::new();

        assert_eq!(bks.len(), 0);
        let bks = add_item(bks, bk1);
        assert_eq!(bks.len(), 1);

        let bks = remove_book(bks, bk2);
        assert_eq!(bks.len(), 0);

        assert_eq!(bks.len(), 0);
        let bks = add_item(bks, bk3);
        assert_eq!(bks.len(), 1);

        let bks = remove_book(bks, bk4);
        assert_eq!(bks.len(), 1);
    }

        #[test]
        fn test_find_borrower_or_book() {
            let br1 = Borrower::new("Borrower1", 1);
            let br2 = Borrower::new("Borrower1", 1);
            let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
            let bk2 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
            let brs1: Vec<Borrower> = Vec::new();
            let bks1: Vec<Book> = Vec::new();
            let brs2: Vec<Borrower> = Vec::new();
            let bks2: Vec<Book> = Vec::new();

            assert_eq!(brs1.len(), 0);
            let brs1 = add_item(brs1, br1);
            assert_eq!(brs1.len(), 1);

            let fnd_br = find_borrower(&brs1, "Borrower1");
            assert_eq!(fnd_br, Some(&Borrower::new("Borrower1", 1)));

            assert_eq!(brs2.len(), 0);
            let brs2 = add_item(brs2, br2);
            assert_eq!(brs2.len(), 1);
            let fnd_br = find_borrower(&brs2, "Borrower11");
            assert_eq!(fnd_br, None);

    //        assert_eq!(bks1.len(), 0);
    //        let bks1 = add_item(bks1, bk1);
    //        assert_eq!(bks1.len(), 1);
    //
    //        let fnd_bk = find_book(bks1, "Title1");
    //        assert_eq!(
    //            fnd_bk,
    //            Some(Book::new(
    //                "Title1",
    //                "Author1",
    //                Some(Borrower::new("Borrower1", 1))
    //            ))
    //        );
    //
    //        assert_eq!(bks2.len(), 0);
    //        let bks2 = add_item(bks2, bk2);
    //        assert_eq!(bks2.len(), 1);
    //
    //        let fnd_bk = find_book(bks2, "Title11");
    //        assert_eq!(fnd_bk, None);
        }
    //
    //    #[test]
    //    fn test_num_books_out() {
    //        let mut lib = Library::new();
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
    //        let bk2 = Book::new("Title2", "Author2", Some(Borrower::new("Borrower2", 2)));
    //        let bk3 = Book::new("Title3", "Author3", Some(Borrower::new("Borrower2", 2)));
    //        lib.add_borrower(br1);
    //        lib.add_borrower(br2);
    //        lib.add_book(bk1);
    //        lib.add_book(bk2);
    //        lib.add_book(bk3);
    //
    //        let fnd_num_bks_2 = lib.num_books_out(&Borrower::new("Borrower2", 2));
    //        assert_eq!(2, fnd_num_bks_2);
    //
    //        let fnd_num_bks_1 = lib.num_books_out(&Borrower::new("Borrower1", 1));
    //        assert_eq!(1, fnd_num_bks_1);
    //
    //        let none_fnd_bks = lib.num_books_out(&Borrower::new("Borrower22", 2));
    //        assert_eq!(0, none_fnd_bks);
    //    }
    //
    //    #[test]
    //    fn test_not_maxed_out() {
    //        let mut lib = Library::new();
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        //        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
    //        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower2", 2)));
    //        let bk2 = Book::new("Title2", "Author2", Some(Borrower::new("Borrower2", 2)));
    //        lib.add_borrower(br1);
    //        lib.add_borrower(br2);
    //        lib.add_book(bk1);
    //        lib.add_book(bk2);
    //
    //        let not_maxed_br1 = lib.not_maxed_out(&Borrower::new("Borrower1", 1));
    //        assert_eq!(true, not_maxed_br1);
    //
    //        let not_maxed_br2 = lib.not_maxed_out(&Borrower::new("Borrower2", 2));
    //        assert_eq!(false, not_maxed_br2);
    //    }
}
