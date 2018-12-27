mod book;
mod borrower;
mod library;

fn main() {
    let mut noodles: String = "noodles".to_string();
    let oodles: &mut str = &mut noodles[1..];
    oodles.make_ascii_uppercase();
    println!("{}", oodles);

    let mut char_list = vec!['y', 'm', 'a', 'q'];
    let mut word_list = vec!["cat".to_owned(), "dog".to_owned()];
    assert!(word_list.contains(&"cat".to_owned()));

    println!("{:?}", char_list);
    library::add_item('e', &mut char_list);
    println!("{:?}", char_list);
    library::add_item('y', &mut char_list);
    println!("{:?}", char_list);
    library::add_item("bird".to_owned(), &mut word_list);
    println!("{:?}", word_list);
    library::add_item("bird".to_owned(), &mut word_list);
    println!("{:?}", word_list);

    let br1 = borrower::Borrower {
        name: String::from("Borrower1"),
        max_books: 1,
    };
    let br2 = borrower::Borrower {
        name: String::from("Borrower2"),
        max_books: 2,
    };
    let br3 = borrower::Borrower {
        name: String::from("Borrower3"),
        max_books: 3,
    };
    let mut borrower_list = vec![br1, br2];
    library::add_item(&br3, &mut borrower_list);
    println!("{:?}", borrower_list);
    library::add_item(br3, &mut borrower_list);
    println!("{:?}", borrower_list);
}
