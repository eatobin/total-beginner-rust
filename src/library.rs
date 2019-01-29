use crate::book::Book;
use crate::borrower::Borrower;

pub fn add_borrower(mut brs: Vec<Borrower>, br: Borrower) -> Vec<Borrower> {
    if !brs.contains(&br) {
        brs.push(br);
    }
    brs
}

pub fn add_book(mut bks: Vec<Book>, bk: Book) -> Vec<Book> {
    if !bks.contains(&bk) {
        bks.push(bk);
    }
    bks
}

pub fn remove_book(mut bks: Vec<Book>, bk: Book) -> Vec<Book> {
    bks.retain(|this_bk| this_bk != &bk);
    bks
}

pub fn find_borrower(brs: Vec<Borrower>, name: &str) -> (Option<Borrower>, Vec<Borrower>) {
    let orig_brs = brs.clone();
    let mut brs_into_iter = brs.into_iter();
    let mbr = brs_into_iter.find(|br| br.get_name() == name);
    (mbr, orig_brs)
}

pub fn find_book(bks: Vec<Book>, title: &str) -> (Option<Book>, Vec<Book>) {
    let orig_bks = bks.clone();
    let mut bks_into_iter = bks.into_iter();
    let mbk = bks_into_iter.find(|bk| bk.get_title() == title);
    (mbk, orig_bks)
}

fn num_books_out(bks: &Vec<Book>, br: &Borrower) -> u8 {
    let mut count: u8 = 0;
    for bk in bks {
        if bk.get_borrower() == Some(br) {
            count += 1;
        }
    }
    count
}

fn not_maxed_out(bks: &Vec<Book>, br: &Borrower) -> bool {
    let out = num_books_out(&bks, br);
    let max = br.get_max_books();
    out < max
}

fn book_not_out(bk: &Book) -> bool {
    bk.get_borrower().is_none()
}

fn book_out(bk: &Book) -> bool {
    bk.get_borrower().is_some()
}

pub fn check_out(
    brs: Vec<Borrower>,
    bks: Vec<Book>,
    name: &str,
    title: &str,
) -> (Vec<Book>, Vec<Borrower>) {
    let orig_bks = bks.clone();
    let (mbr, brs) = find_borrower(brs, name);
    let (mbk, bks) = find_book(bks, title);
    if mbr.is_some()
        && mbk.is_some()
        && not_maxed_out(&bks, &mbr.clone().unwrap())
        && book_not_out(&mbk.clone().unwrap())
    {
        let bk = mbk.unwrap();
        let new_book = bk.clone().set_borrower(mbr);
        let fewer_bks = remove_book(bks, bk);
        let new_bks = add_book(fewer_bks, new_book);
        (new_bks, brs)
    } else {
        (orig_bks, brs)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_library() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower1", 1);
        let mut brs1: Vec<Borrower> = Vec::new();

        // add borrower
        assert_eq!(brs1.len(), 0);
        let brs1 = add_borrower(brs1.clone(), br1.clone());
        assert_eq!(brs1.len(), 1);
        let brs1 = add_borrower(brs1, br2.clone());
        assert_eq!(brs1.len(), 1);

        // add book
        let bk1 = Book::new("Title1", "Author1", Some(br1));
        let bk2 = Book::new("Title1", "Author1", Some(br2));
        let mut bks1: Vec<Book> = Vec::new();
        assert_eq!(bks1.len(), 0);
        let bks1 = add_book(bks1, bk1.clone());
        assert_eq!(bks1.len(), 1);
        let bks1 = add_book(bks1, bk2.clone());
        assert_eq!(bks1.len(), 1);

        // remove book
        let bk3 = Book::new("Title3", "Author3", Some(Borrower::new("Borrower3", 3)));
        let bk4 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let mut bks1: Vec<Book> = Vec::new();

        assert_eq!(bks1.len(), 0);
        let bks1 = add_book(bks1, bk1.clone());
        assert_eq!(bks1.len(), 1);
        let bks1 = remove_book(bks1.clone(), bk2);
        assert_eq!(bks1.len(), 0);

        let bks1 = add_book(bks1, bk3);
        assert_eq!(bks1.len(), 1);

        let bks1 = remove_book(bks1, bk4);
        assert_eq!(bks1.len(), 1);
    }

    #[test]
    fn test_find_borrower_or_book() {
        let br1 = Borrower::new("Borrower1", 1);
        let brs1: Vec<Borrower> = Vec::new();

        assert_eq!(brs1.len(), 0);
        let brs1 = add_borrower(brs1, br1);
        assert_eq!(brs1.len(), 1);

        let (fnd_br, brs1) = find_borrower(brs1, "Borrower1");
        assert_eq!(fnd_br, Some(Borrower::new("Borrower1", 1)));
        assert_eq!(brs1.len(), 1);

        let (fnd_br, brs1) = find_borrower(brs1, "Borrower11");
        assert_eq!(fnd_br, None);
        assert_eq!(brs1.len(), 1);

        let br1 = Borrower::new("Borrower1", 1);
        let sbr1 = Some(br1);
        let bk1 = Book::new("Title1", "Author1", sbr1);
        let bks1: Vec<Book> = Vec::new();

        assert_eq!(bks1.len(), 0);
        let bks1 = add_book(bks1, bk1);
        assert_eq!(bks1.len(), 1);

        let (fnd_bk, bks1) = find_book(bks1, "Title1");
        assert_eq!(
            fnd_bk,
            Some(Book::new(
                "Title1",
                "Author1",
                Some(Borrower::new("Borrower1", 1)),
            ))
        );
        assert_eq!(bks1.len(), 1);

        let (fnd_bk, bks1) = find_book(bks1, "Title11");
        assert_eq!(fnd_bk, None);
        assert_eq!(bks1.len(), 1);
    }

    #[test]
    fn test_num_books_out() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let br3 = Borrower::new("Borrower2", 2);
        let sbr1 = Some(br1);
        let sbr2 = Some(br2);
        let sbr3 = Some(br3);
        let bk1 = Book::new("Title1", "Author1", sbr1);
        let bk2 = Book::new("Title2", "Author2", sbr2);
        let bk3 = Book::new("Title3", "Author3", sbr3);
        let bks1: Vec<Book> = Vec::new();
        let bks1 = add_book(bks1, bk1);
        let bks1 = add_book(bks1, bk2);
        let bks1 = add_book(bks1, bk3);
        assert_eq!(bks1.len(), 3);

        let fnd_num_bks_2 = num_books_out(&bks1, &Borrower::new("Borrower2", 2));
        assert_eq!(fnd_num_bks_2, 2);

        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let br3 = Borrower::new("Borrower2", 2);
        let sbr1 = Some(br1);
        let sbr2 = Some(br2);
        let sbr3 = Some(br3);
        let bk1 = Book::new("Title1", "Author1", sbr1);
        let bk2 = Book::new("Title2", "Author2", sbr2);
        let bk3 = Book::new("Title3", "Author3", sbr3);
        let bks1: Vec<Book> = Vec::new();
        let bks1 = add_book(bks1, bk1);
        let bks1 = add_book(bks1, bk2);
        let bks1 = add_book(bks1, bk3);

        let fnd_num_bks_1 = num_books_out(&bks1, &Borrower::new("Borrower1", 1));
        assert_eq!(fnd_num_bks_1, 1);

        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let br3 = Borrower::new("Borrower2", 2);
        let sbr1 = Some(br1);
        let sbr2 = Some(br2);
        let sbr3 = Some(br3);
        let bk1 = Book::new("Title1", "Author1", sbr1);
        let bk2 = Book::new("Title2", "Author2", sbr2);
        let bk3 = Book::new("Title3", "Author3", sbr3);
        let bks1: Vec<Book> = Vec::new();
        let bks1 = add_book(bks1, bk1);
        let bks1 = add_book(bks1, bk2);
        let bks1 = add_book(bks1, bk3);

        let none_fnd_bks = num_books_out(&bks1, &Borrower::new("Borrower22", 2));
        assert_eq!(none_fnd_bks, 0);
    }

    #[test]
    fn test_not_maxed_out() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let sbr1 = Some(br1);
        let sbr2 = Some(br2);
        let bk1 = Book::new("Title1", "Author1", sbr1);
        let bk2 = Book::new("Title1", "Author1", sbr2);
        let bks1: Vec<Book> = Vec::new();
        let bks1 = add_book(bks1, bk1);
        let bks1 = add_book(bks1, bk2);

        let not_maxed_br1 = not_maxed_out(&bks1, &Borrower::new("Borrower1", 1));
        assert_eq!(not_maxed_br1, false);

        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let sbr1 = Some(br1);
        let sbr2 = Some(br2);
        let mut bk1 = Book::new("Title1", "Author1", sbr1);
        let mut bk2 = Book::new("Title1", "Author1", sbr2);
        let bks1: Vec<Book> = Vec::new();
        let bks1 = add_book(bks1, bk1);
        let bks1 = add_book(bks1, bk2);

        let not_maxed_br2 = not_maxed_out(&bks1, &Borrower::new("Borrower2", 2));
        assert_eq!(not_maxed_br2, true);
    }

    #[test]
    fn test_check_out() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let brs1: Vec<Borrower> = Vec::new();
        let brs1 = add_borrower(brs1, br1.clone());
        let brs1 = add_borrower(brs1, br2.clone());

        let sbr1 = Some(br1);
        let sbr2 = Some(br2);
        let bk1 = Book::new("Title1", "Author1", sbr1);
        let bk2 = Book::new("Title2", "Author2", None);
        let bk3 = Book::new("Title2", "Author2", sbr2);

        let bks1: Vec<Book> = Vec::new();
        let bks1 = add_book(bks1, bk1.clone());
        let bks1 = add_book(bks1, bk2.clone());
        let bks2: Vec<Book> = Vec::new();
        let bks2 = add_book(bks2, bk1);
        let bks2 = add_book(bks2, bk3);

        // check-out-pass-test
        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "Borrower2", "Title2");
        assert_eq!(ret_bks, bks2);

        // check-out-fail-checked-out-test
        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "Borrower2", "Title1");
        assert_eq!(ret_bks, bks1);

        // check-out-fail-bad-book-test
        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "Borrower2", "NoTitle");
        assert_eq!(ret_bks, bks1);

        // check-out-fail-bad-borrower-test
        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "NoName", "Title2");
        assert_eq!(ret_bks, bks1);

        // check-out-fail-over-limit-test
        let (ret_bks, brs1) = check_out(brs1, bks1.clone(), "Borrower1", "Title2");
        assert_eq!(ret_bks, bks1);
    }
}
