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

    pub fn find_borrower(&self, name: &str) -> Option<&Borrower> {
        self.borrowers
            .iter()
            .find(|br| Borrower::get_name(br) == name)
    }

    pub fn find_book(&self, title: &str) -> Option<&Book> {
        self.books
            .iter()
            .find(|bk| Book::get_title(bk) == title)
    }

    fn num_books_out(&self, br: &Borrower) -> u8 {
        let mut count: u8 = 0;
        for nxt_bk in &(self.books) {
            if nxt_bk.get_borrower().as_ref() == Some(br) {
                count += 1;
            }
        }
        count
    }

    fn not_maxed_out(&self, br: &Borrower) -> bool {
        let out = Library::num_books_out(&self, br);
        let max = br.get_max_books();
        out < max
    }

    fn book_not_out(bk: &Book) -> bool {
        bk.get_borrower().is_none()
    }

    fn book_out(bk: &Book) -> bool {
        bk.get_borrower().is_some()
    }

        pub fn check_in(self, name: &str, title: &str) -> Library {
            let lib = self;
            let mbr = lib.find_borrower(name);
            let mbk = lib.find_book(title);
//            let mbr = Library::find_borrower(&self, name);
//            let mbk = Library::find_book(&self, title);
//            if mbk.is_some() && mbr.is_some() && Library::not_maxed_out(self, mbr.unwrap()) && Library::book_not_out(mbk.unwrap()) { }
            if mbr.is_some() && mbk.is_some() && lib.not_maxed_out(mbr.unwrap()) && Library::book_not_out(mbk.unwrap()) {
                let new_book = Book{borrower: mbr, ..*mbk});
                lib
            }else { lib }
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

        assert_eq!(fnd_br, Some(&Borrower::new("Borrower1", 1)));

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
    fn test_num_books_out() {
        let mut lib = Library::new();
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let bk2 = Book::new("Title2", "Author2", Some(Borrower::new("Borrower2", 2)));
        let bk3 = Book::new("Title3", "Author3", Some(Borrower::new("Borrower2", 2)));
        lib.add_borrower(br1);
        lib.add_borrower(br2);
        lib.add_book(bk1);
        lib.add_book(bk2);
        lib.add_book(bk3);

        let fnd_num_bks_2 = lib.num_books_out(&Borrower::new("Borrower2", 2));
        assert_eq!(2, fnd_num_bks_2);

        let fnd_num_bks_1 = lib.num_books_out(&Borrower::new("Borrower1", 1));
        assert_eq!(1, fnd_num_bks_1);

        let none_fnd_bks = lib.num_books_out(&Borrower::new("Borrower22", 2));
        assert_eq!(0, none_fnd_bks);
    }

    #[test]
    fn test_not_maxed_out() {
        let mut lib = Library::new();
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        //        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower2", 2)));
        let bk2 = Book::new("Title2", "Author2", Some(Borrower::new("Borrower2", 2)));
        lib.add_borrower(br1);
        lib.add_borrower(br2);
        lib.add_book(bk1);
        lib.add_book(bk2);

        let not_maxed_br1 = lib.not_maxed_out(&Borrower::new("Borrower1", 1));
        assert_eq!(true, not_maxed_br1);

        let not_maxed_br2 = lib.not_maxed_out(&Borrower::new("Borrower2", 2));
        assert_eq!(false, not_maxed_br2);
    }
}
