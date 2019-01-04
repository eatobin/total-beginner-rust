use crate::book::Book;
use crate::borrower::Borrower;

pub fn add_item<T: PartialEq>(x: T, mut xs: Vec<T>) -> Vec<T> {
    if xs.contains(&x) {
        xs
    } else {
        xs.push(x);
        xs
    }
}

pub fn remove_book(bk: Book, mut bks: Vec<Book>) -> Vec<Book> {
    bks.retain(|i| i != &bk);
    bks
}

pub fn find_item<'a, T>(tgt: &str, coll: &'a Vec<&T>, f: &Fn(&'a T) -> &str) -> Option<&'a &'a T> {
    coll.iter().find(|&&i| f(i) == tgt)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_item() {
        let br = Borrower::new("Borrower1", 1);
        let mut brs: Vec<Borrower> = Vec::new();

        brs = add_item(br, brs);
        assert_eq!(vec![Borrower::new("Borrower1", 1),], brs);

        let br_dup = Borrower::new("Borrower1", 1);
        brs = add_item(br_dup, brs);
        assert_eq!(vec![Borrower::new("Borrower1", 1),], brs);

        let bk = Book::new("Title1", "Author1", None);
        let mut bks: Vec<Book> = Vec::new();

        bks = add_item(bk, bks);
        assert_eq!(vec![Book::new("Title1", "Author1", None)], bks);

        let bk_dup = Book::new("Title1", "Author1", None);
        bks = add_item(bk_dup, bks);
        assert_eq!(vec![Book::new("Title1", "Author1", None)], bks);
    }

    #[test]
    fn test_remove_book() {
        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let bk2 = Book::new("Title2", "Author2", None);
        let bk3 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let bk4 = Book::new("Title2", "Author2", None);
        let bk5 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
        let mut bks1: Vec<Book> = vec![bk1, bk2];
        let bks2: Vec<Book> = vec![bk3, bk4];

        bks1 = remove_book(Book::new("Title22", "Author2", None), bks1);
        assert_eq!(bks1, bks2);

        let bks2: Vec<Book> = vec![bk5];

        bks1 = remove_book(Book::new("Title2", "Author2", None), bks1);
        assert_eq!(bks1, bks2);
    }

    #[test]
    fn test_find_item() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower2", 2);
        let mut brs: Vec<&Borrower> = Vec::new();
        brs = add_item(&br1, brs);
        brs.push(&br2);

        let fnd_br = find_item("Borrower1", &brs, &Borrower::get_name);
        assert_eq!(fnd_br, Some(&&br1));

        let fnd_br: Option<&&Borrower> = find_item("Borrower11", &brs, &Borrower::get_name);
        assert_eq!(fnd_br, None);

        let bk1 = Book::new("Title1", "Author1", None);
        let bk2 = Book::new("Title2", "Author2", None);
        let mut bks: Vec<&Book> = Vec::new();
        bks.push(&bk1);
        bks.push(&bk2);

        let fnd_bk = find_item("Title1", &bks, &Book::get_title);
        assert_eq!(fnd_bk, Some(&&bk1));

        let fnd_bk = find_item("Title11", &bks, &Book::get_title);
        assert_eq!(fnd_bk, None);
    }
}
