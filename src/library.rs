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

pub fn find_item<'a, T>(tgt: &str, coll: &'a mut Vec<T>, f: &Fn(&T) -> &str) -> Option<&'a mut T> {
    let mut coll_i = coll.into_iter();
    let r = coll_i.find(|i| f(i) == tgt);
    r
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_item() {
        let br1 = Borrower {
            name: String::from("Borrower1"),
            max_books: 1,
        };
        let mut brs: Vec<Borrower> = Vec::new();

        brs = add_item(br1, brs);
        assert_eq!(
            vec![Borrower {
                name: String::from("Borrower1"),
                max_books: 1,
            }, ],
            brs
        );

        brs = add_item(Borrower::new("Borrower1", 1), brs);
        assert_eq!(
            vec![Borrower {
                name: String::from("Borrower1"),
                max_books: 1,
            }, ],
            brs
        );

        let bk1 = Book {
            title: "Title1".to_owned(),
            author: String::from("Author1"),
            borrower: None,
        };
        let mut bks: Vec<Book> = Vec::new();

        bks = add_item(bk1, bks);
        assert_eq!(
            vec![Book {
                title: "Title1".to_owned(),
                author: "Author1".to_owned(),
                borrower: None,
            }, ],
            bks
        );

        bks = add_item(Book::new("Title1", "Author1"), bks);
        assert_eq!(
            vec![Book {
                title: "Title1".to_owned(),
                author: "Author1".to_owned(),
                borrower: None,
            }, ],
            bks
        );
    }

    #[test]
    fn test_remove_book() {
        let mut bks1 = vec![
            Book {
                title: "Title1".to_owned(),
                author: "Author1".to_owned(),
                borrower: None,
            },
            Book {
                title: "Title2".to_owned(),
                author: "Author2".to_owned(),
                borrower: None,
            },
        ];
        bks1 = remove_book(
            Book {
                title: "Title1".to_owned(),
                author: "Author1".to_owned(),
                borrower: None,
            },
            bks1,
        );
        let bks2 = vec![Book {
            title: "Title2".to_owned(),
            author: "Author2".to_owned(),
            borrower: None,
        }];
        assert_eq!(bks1, bks2);

        bks1 = remove_book(
            Book {
                title: "Title1".to_owned(),
                author: "Author1".to_owned(),
                borrower: None,
            },
            bks1,
        );
        assert_eq!(bks1, bks2);
    }

    #[test]
    fn test_find_item() {
        let br1 = Borrower {
            name: String::from("Borrower1"),
            max_books: 1,
        };
        let br2 = Borrower {
            name: String::from("Borrower2"),
            max_books: 2,
        };

        let mut brl: Vec<Borrower> = Vec::new();
        brl.push(br1);
        brl.push(br2);

        let bk1 = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };

        let bk2 = Book {
            title: "Title2".to_owned(),
            author: "Author2".to_owned(),
            borrower: Some(Borrower {
                name: String::from("Borrower2"),
                max_books: 2,
            }),
        };

        let mut bkl: Vec<Book> = Vec::new();
        bkl.push(bk1);
        bkl.push(bk2);

        let exp_br_item_1 = &mut Borrower {
            name: String::from("Borrower1"),
            max_books: 1,
        };
        let br_item_1 = find_item("Borrower1", &mut brl, &Borrower::get_name).unwrap();

        assert_eq!(exp_br_item_1, br_item_1)
    }
}
