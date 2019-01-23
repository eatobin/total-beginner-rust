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

pub fn find_borrower(brs: Vec<Borrower>, name: &str) -> Option<Borrower> {
    let mut brs_into_iter = brs.into_iter();
    brs_into_iter.find(|br| br.get_name() == name)
}

//pub fn find_borrower<'a>(brs: &'a Vec<&'a Borrower>, name: &str) -> Option<&'a &'a Borrower> {
//    let mut brs_into_iter = brs.into_iter();
//    brs_into_iter.find(|br| Borrower::get_name(br) == name)
//}

pub fn find_book(bks: Vec<Book>, title: &str) -> Option<Book> {
    let mut bks_into_iter = bks.into_iter();
    bks_into_iter.find(|bk| bk.get_title() == title)
}

//pub fn find_book<'a>(bks: &'a Vec<&'a mut Book<'a>>, title: &str) -> Option<&'a &'a mut Book<'a>> {
//    let mut bks_into_iter = bks.into_iter();
//    bks_into_iter.find(|bk| Book::get_title(bk) == title)
//}

fn num_books_out<'a>(bks: &'a Vec<Book>, br: &'a Borrower) -> u8 {
    let mut count: u8 = 0;
    for nxt_bk in bks {
        if nxt_bk.get_borrower().as_ref() == Some(&br) {
            count += 1;
        }
    }
    count
}
//
//fn not_maxed_out<'a>(bks: &'a Vec<&'a mut Book<'a>>, br: &'a Borrower) -> bool {
//    let out = num_books_out(&bks, br);
//    let max = br.get_max_books();
//    out < max
//}
//
//fn book_not_out<'a>(bk: &'a Book) -> bool {
//    bk.get_borrower().is_none()
//}
//
//fn book_out<'a>(bk: &'a Book) -> bool {
//    bk.get_borrower().is_some()
//}
//
//pub fn check_out<'a>(
//    brs: &'a Vec<&'a Borrower>,
//    bks: &'a Vec<&'a mut Book<'a>>,
//    name: &str,
//    title: &str,
//) -> &'a Vec<&'a mut Book<'a>> {
//        let mbr = find_borrower(&brs, name);
//        let mut mbk = find_book(&bks, title);
//        if (&mbr).is_some()
//            && (&mbk).is_some()
//            && not_maxed_out(bks, (&mbr).unwrap())
//            && book_not_out((&mbk).unwrap())
//        {
////          let new_book = &(mbk.unwrap().to_owned().set_borrower(Some(mbr.unwrap())));
//            let new_book = mbk.unwrap().set_borrower(Some(mbr.unwrap()));
//            unimplemented!()
//        }
//    unimplemented!()
//}
//
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_borrower_or_book() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower1", 1);
        let mut brs: Vec<Borrower> = Vec::new();

        assert_eq!(brs.len(), 0);
        let brs = add_borrower(brs, br1);
        assert_eq!(brs.len(), 1);
        let brs = add_borrower(brs, br2);
        assert_eq!(brs.len(), 1);

        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower1", 1);
        let mut bk1 = Book::new("Title1", "Author1", Some(br1));
        let mut bk2 = Book::new("Title1", "Author1", Some(br2));
        let mut bks: Vec<Book> = Vec::new();
        assert_eq!(bks.len(), 0);
        let bks = add_book(bks, bk1);
        assert_eq!(bks.len(), 1);
        let bks = add_book(bks, bk2);
        assert_eq!(bks.len(), 1);
    }

    //    //    #[test]
    //    //    fn test_remove_book() {
    //    //        let bk1 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
    //    //        let bk2 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
    //    //        let bk3 = Book::new("Title3", "Author3", Some(Borrower::new("Borrower3", 3)));
    //    //        let bk4 = Book::new("Title1", "Author1", Some(Borrower::new("Borrower1", 1)));
    //    //        let mut bks: Vec<Book> = Vec::new();
    //    //
    //    //        assert_eq!(bks.len(), 0);
    //    //        let bks = add_item(bks, bk1);
    //    //        assert_eq!(bks.len(), 1);
    //    //
    //    //        let bks = remove_book(bks, bk2);
    //    //        assert_eq!(bks.len(), 0);
    //    //
    //    //        assert_eq!(bks.len(), 0);
    //    //        let bks = add_item(bks, bk3);
    //    //        assert_eq!(bks.len(), 1);
    //    //
    //    //        let bks = remove_book(bks, bk4);
    //    //        assert_eq!(bks.len(), 1);
    //    //    }

    #[test]
    fn test_find_borrower_or_book() {
        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower1", 1);
        let brs1: Vec<Borrower> = Vec::new();
        let brs2: Vec<Borrower> = Vec::new();

        assert_eq!(brs1.len(), 0);
        let brs1 = add_borrower(brs1, br1);
        assert_eq!(brs1.len(), 1);

        let fnd_br = find_borrower(brs1, "Borrower1");
        assert_eq!(fnd_br, Some(Borrower::new("Borrower1", 1)));

        assert_eq!(brs2.len(), 0);
        let brs2 = add_borrower(brs2, br2);
        assert_eq!(brs2.len(), 1);
        let fnd_br = find_borrower(brs2, "Borrower11");
        assert_eq!(fnd_br, None);

        let br1 = Borrower::new("Borrower1", 1);
        let br2 = Borrower::new("Borrower1", 1);
        let sbr1 = Some(br1);
        let sbr2 = Some(br2);
        let mut bk1 = Book::new("Title1", "Author1", sbr1);
        let mut bk2 = Book::new("Title1", "Author1", sbr2);

        let mut bks1: Vec<Book> = Vec::new();
        let mut bks2: Vec<Book> = Vec::new();

        assert_eq!(bks1.len(), 0);
        let bks1 = add_book(bks1, bk1);
        assert_eq!(bks1.len(), 1);

        let fnd_bk = find_book(bks1, "Title1");
        assert_eq!(
            fnd_bk,
            Some(Book::new(
                "Title1",
                "Author1",
                Some(Borrower::new("Borrower1", 1)),
            ))
        );

        assert_eq!(bks2.len(), 0);
        let mut bks2 = add_book(bks2, bk2);
        assert_eq!(bks2.len(), 1);

        let fnd_bk = find_book(bks2, "Title11");
        assert_eq!(fnd_bk, None);
    }

    //    #[test]
    //    fn test_num_books_out() {
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let br3 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(&br1);
    //        let sbr2 = Some(&br2);
    //        let sbr3 = Some(&br3);
    //        let mut bk1 = Book::new("Title1", "Author1", sbr1);
    //        let mut bk2 = Book::new("Title2", "Author2", sbr2);
    //        let mut bk3 = Book::new("Title3", "Author3", sbr3);
    //        let bks1: Vec<&mut Book> = Vec::new();
    //        let bks1 = add_book(bks1, &mut bk1);
    //        let bks1 = add_book(bks1, &mut bk2);
    //        let bks1 = add_book(bks1, &mut bk3);
    //        assert_eq!(bks1.len(), 3);
    //
    //        let fnd_num_bks_2 = num_books_out(&bks1, &Borrower::new("Borrower2", 2));
    //        assert_eq!(fnd_num_bks_2, 2);
    //
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let br3 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(&br1);
    //        let sbr2 = Some(&br2);
    //        let sbr3 = Some(&br3);
    //        let mut bk1 = Book::new("Title1", "Author1", sbr1);
    //        let mut bk2 = Book::new("Title2", "Author2", sbr2);
    //        let mut bk3 = Book::new("Title3", "Author3", sbr3);
    //        let bks1: Vec<&mut Book> = Vec::new();
    //        let bks1 = add_book(bks1, &mut bk1);
    //        let bks1 = add_book(bks1, &mut bk2);
    //        let bks1 = add_book(bks1, &mut bk3);
    //
    //        let fnd_num_bks_1 = num_books_out(&bks1, &Borrower::new("Borrower1", 1));
    //        assert_eq!(fnd_num_bks_1, 1);
    //
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let br3 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(&br1);
    //        let sbr2 = Some(&br2);
    //        let sbr3 = Some(&br3);
    //        let mut bk1 = Book::new("Title1", "Author1", sbr1);
    //        let mut bk2 = Book::new("Title2", "Author2", sbr2);
    //        let mut bk3 = Book::new("Title3", "Author3", sbr3);
    //        let bks1: Vec<&mut Book> = Vec::new();
    //        let bks1 = add_book(bks1, &mut bk1);
    //        let bks1 = add_book(bks1, &mut bk2);
    //        let bks1 = add_book(bks1, &mut bk3);
    //
    //        let none_fnd_bks = num_books_out(&bks1, &Borrower::new("Borrower22", 2));
    //        assert_eq!(none_fnd_bks, 0);
    //    }
    //
    //    #[test]
    //    fn test_not_maxed_out() {
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(&br1);
    //        let sbr2 = Some(&br2);
    //        let mut bk1 = Book::new("Title1", "Author1", sbr1);
    //        let mut bk2 = Book::new("Title1", "Author1", sbr2);
    //        let bks1: Vec<&mut Book> = Vec::new();
    //        let bks1 = add_book(bks1, &mut bk1);
    //        let bks1 = add_book(bks1, &mut bk2);
    //
    //        let not_maxed_br1 = not_maxed_out(&bks1, &Borrower::new("Borrower1", 1));
    //        assert_eq!(false, not_maxed_br1);
    //
    //        let br1 = Borrower::new("Borrower1", 1);
    //        let br2 = Borrower::new("Borrower2", 2);
    //        let sbr1 = Some(&br1);
    //        let sbr2 = Some(&br2);
    //        let mut bk1 = Book::new("Title1", "Author1", sbr1);
    //        let mut bk2 = Book::new("Title1", "Author1", sbr2);
    //        let bks1: Vec<&mut Book> = Vec::new();
    //        let bks1 = add_book(bks1, &mut bk1);
    //        let bks1 = add_book(bks1, &mut bk2);
    //
    //        let not_maxed_br2 = not_maxed_out(&bks1, &Borrower::new("Borrower2", 2));
    //        assert_eq!(true, not_maxed_br2);
    //    }
}
