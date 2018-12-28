#![allow(unused)]

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
    char_list = library::add_item('e', char_list);
    println!("{:?}", char_list);

    word_list = library::add_item("bird".to_owned(), word_list);
    println!("{:?}", word_list);
    word_list = library::add_item("bird".to_owned(), word_list);
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

    let mut bl: Vec<borrower::Borrower> = Vec::new();
    bl.push(br1);
    bl.push(br2);
    println!("{:?}", bl);
    bl = library::add_item(br3, bl);
    println!("{:?}", bl);
    bl = library::add_item(borrower::Borrower::new("Borrower3", 3), bl);
    println!("{:?}", bl);

    //    br1 = borrower::Borrower {
    //        name: String::from("Borrower11"),
    //        max_books: 11,
    //    };
    //    println!("{:?}", br1);
    //
    //    let mut s = "me".to_string();
    //    let t = s;
    //    s = "you".to_string();
    //    println!("{}", t);
    //    println!("{}", s);
}
