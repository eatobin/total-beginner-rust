#![allow(unused)]

mod book;
mod borrower;
mod library;
use crate::book::Book;
use crate::book::Borrower;

fn main() {
    // let mut noodles: String = "noodles".to_string();
    // let oodles: &mut str = &mut noodles[1..];
    // oodles.make_ascii_uppercase();
    // println!("{}", oodles);

    // let mut char_list = vec!['y', 'm', 'a', 'q'];
    // let mut word_list = vec!["cat".to_owned(), "dog".to_owned()];
    // assert!(word_list.contains(&"cat".to_owned()));

    // println!("{:?}", char_list);
    // char_list = library::add_item('e', char_list);
    // println!("{:?}", char_list);

    // word_list = library::add_item("bird".to_owned(), word_list);
    // println!("{:?}", word_list);
    // word_list = library::add_item("bird".to_owned(), word_list);
    // println!("{:?}", word_list);

    let br1 = Borrower {
        name: String::from("Borrower1"),
        max_books: 1,
    };
    let br2 = Borrower {
        name: String::from("Borrower2"),
        max_books: 2,
    };
    let br3 = Borrower {
        name: String::from("Borrower3"),
        max_books: 3,
    };

    let mut bl: Vec<Borrower> = Vec::new();
    bl.push(br2);
    bl.push(br3);
    // println!("{:?}", bl);
    // bl = library::add_item(br3, bl);
    // println!("{:?}", bl);
    // bl = library::add_item(Borrower::new("Borrower3", 3), bl);
    // println!("{:?}", bl);

    //    br1 = Borrower {
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
    // fun_test(5, &times2);
    // println!("{}", is_br(&br1, &Borrower::get_name, "Borrower1"));
    println!(
        "{:?}",
        find_borrower("Borrower2", &mut bl, &Borrower::get_name)
    );
    println!(
        "{:?}",
        find_borrower("Borrower22", &mut bl, &Borrower::get_name)
    );
    // println!("{:?}", find_item("Borrower2", &mut bl, &Borrower::get_name));

    // let bk1 = Book {
    //     title: "Title1".to_owned(),
    //     author: "Author1".to_owned(),
    //     borrower: None,
    // };

    // let bk2 = Book {
    //     title: "title2".to_owned(),
    //     author: "Author1".to_owned(),
    //     borrower: None,
    // };

    // let mut bkl: Vec<Book> = Vec::new();
    // bkl.push(bk1);
    // bkl.push(bk2);

    // println!("{:?}", find_item("Title11", &mut bkl, &Book::get_title));

    // let vec2 = vec![4, 5, 6];
    // let mut into_iter = vec2.into_iter();
    // println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 5));

    // let mut bkl_i = bkl.into_iter();
    // println!(
    //     "Find 2 in bkl_i: {:?}",
    //     bkl_i.find(|x| x
    //         == &Book {
    //             title: "Title11".to_owned(),
    //             author: "Author1".to_owned(),
    //             borrower: None,
    //         })
    // );
}

fn fun_test(value: i32, f: &Fn(i32) -> i32) -> i32 {
    println!("{}", f(value));
    value
}

fn times2(value: i32) -> i32 {
    2 * value
}

fn is_br(br: &Borrower, f: &Fn(&Borrower) -> &str, target: &str) -> bool {
    f(br) == target
}

fn find_borrower<'a>(
    tgt: &str,
    coll: &'a mut Vec<Borrower>,
    f: &Fn(&Borrower) -> &str,
) -> Option<&'a mut Borrower> {
    let mut coll_i = coll.into_iter();
    let r = coll_i.find(|i| f(i) == tgt);
    r
}

fn find_item<'a, T>(tgt: &str, coll: &'a mut Vec<T>, f: &Fn(&T) -> &str) -> Option<&'a T> {
    coll.retain(|i| f(i) == tgt);
    if coll.is_empty() {
        None
    } else {
        Some(&coll[0])
    }
}
