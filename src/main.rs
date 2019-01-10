//#![allow(unused)]
//
//mod book;
//mod borrower;
//mod library;
//
//use crate::book::Book;
//use crate::borrower::Borrower;
//use std::collections::HashSet;
//use std::str;
//
//#[derive(Hash, Eq, PartialEq, Debug)]
//struct Viking {
//    name: String,
//    power: usize,
//}
//
fn main() {
    // let mut noodles: String = "noodles".to_string();
    // let oodles: &mut str = &mut noodles[1..];
    // oodles.make_ascii_uppercase();
    // println!("{}", oodles);
}
//
//    // let mut char_list = vec!['y', 'm', 'a', 'q'];
//    // let mut word_list = vec!["cat".to_owned(), "dog".to_owned()];
//    // assert!(word_list.contains(&"cat".to_owned()));
//
//    // println!("{:?}", char_list);
//    // char_list = library::add_item('e', char_list);
//    // println!("{:?}", char_list);
//
//    // word_list = library::add_item("bird".to_owned(), word_list);
//    // println!("{:?}", word_list);
//    // word_list = library::add_item("bird".to_owned(), word_list);
//    //    // println!("{:?}", word_list);
//    //
//    //    let br1 = Borrower {
//    //        name: String::from("Borrower1"),
//    //        max_books: 1,
//    //    };
//    //    let br2 = Borrower {
//    //        name: String::from("Borrower2"),
//    //        max_books: 2,
//    //    };
//    //    let br3 = Borrower {
//    //        name: String::from("Borrower3"),
//    //        max_books: 3,
//    //    };
//    //
//    //    let mut bl: Vec<Borrower> = Vec::new();
//    //    bl.push(br2);
//    //    bl.push(br3);
//    //    // println!("{:?}", bl);
//    // bl = library::add_item(br3, bl);
//    // println!("{:?}", bl);
//    // bl = library::add_item(Borrower::new("Borrower3", 3), bl);
//    // println!("{:?}", bl);
//
//    //    br1 = Borrower {
//    //        name: String::from("Borrower11"),
//    //        max_books: 11,
//    //    };
//    //    println!("{:?}", br1);
//    //
//    //    let mut s = "me".to_string();
//    //    let t = s;
//    //    s = "you".to_string();
//    //    println!("{}", t);
//    //    println!("{}", s);
//    // fun_test(5, &times2);
//    // println!("{}", is_br(&br1, &Borrower::get_name, "Borrower1"));
//    //    println!(
//    //        "{:?}",
//    //        find_borrower("Borrower2", &mut bl, &Borrower::get_name)
//    //    );
//    //    println!(
//    //        "{:?}",
//    //        find_borrower("Borrower22", &mut bl, &Borrower::get_name)
//    //    );
//    //    println!("{:?}", find_item("Borrower2", &mut bl, &Borrower::get_name));
//    //
//    //    let bk1 = Book {
//    //        title: "Title1".to_owned(),
//    //        author: "Author1".to_owned(),
//    //        borrower: None,
//    //    };
//    //
//    //    let bk2 = Book {
//    //        title: "Title2".to_owned(),
//    //        author: "Author2".to_owned(),
//    //        borrower: None,
//    //    };
//    //
//    //    let mut bkl: Vec<Book> = Vec::new();
//    //    bkl.push(bk1);
//    //    bkl.push(bk2);
//    //
//    //    println!("{:?}", find_item("Title1", &mut bkl, &Book::get_title));
//    //    println!("{:?}", find_item("Title2", &mut bkl, &Book::get_title));
//    //    println!("{:?}", find_item("Title11", &mut bkl, &Book::get_title));
//    //    assert_eq!(
//    //        find_item("Title2", &mut bkl, &Book::get_title),
//    //        Some(&Book {
//    //            title: "Title2".to_owned(),
//    //            author: "Author2".to_owned(),
//    //            borrower: None,
//    //        })
//    //    );
//    // let vec2 = vec![4, 5, 6];
//    // let mut into_iter = vec2.into_iter();
//    // println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 5));
//
//    // let mut bkl_i = bkl.into_iter();
//    // println!(
//    //     "Find 2 in bkl_i: {:?}",
//    //     bkl_i.find(|x| x
//    //         == &Book {
//    //             title: "Title11".to_owned(),
//    //             author: "Author1".to_owned(),
//    //             borrower: None,
//    //         })
//    // );
//
//    //    let a = ["lol", "NaN", "n", "t"];
//    //
//    //    let mut first_number: Option<u16> = a.iter().find_map(|s| s.parse().ok());
//    //    assert_eq!(first_number, None);
//    //
//    //    let mut first_book: Option<&Book> = bkl.iter().find(|&i| {
//    //        i == &Book {
//    //            title: "Title2".to_owned(),
//    //            author: "Author2".to_owned(),
//    //            borrower: None,
//    //        }
//    //    });
//    //    assert_eq!(
//    //        first_book,
//    //        Some(&Book {
//    //            title: "Title2".to_owned(),
//    //            author: "Author2".to_owned(),
//    //            borrower: None,
//    //        })
//    //    )
//
//    let v = &vec![1, 2, 3, 4, 5, 6];
//    let jj = v.into_iter().filter(|&i| i % 2 == 0).collect::<Vec<_>>();
//    println!("{:?}", jj);
//    println!("{:?}", v);
//
//    let w = vec!["do", "the", "best"];
//    println!("{:?}", find_string(4, &w, &str::len));
//
//    let br1 = Borrower::new("Borrower1", 1);
//    let br2 = Borrower::new("Borrower2", 2);
//    let mut brs: HashSet<Borrower> = HashSet::new();
//    brs.insert(br1);
//    brs.insert(br2);
//
//    println!(
//        "{:?}",
//        find_borrower("Borrower1", &brs, &Borrower::get_name)
//    );
//
//    println!("{:?}", brs.iter().find(|&br| br.get_name() == "Borrower1"));
//
//    //    assert_eq!(brs, vec![&br1, &br2]);
//
//    //    println!("{:?}", find_item("Borrower2", &brs, &Borrower::get_name));
//    //    println!("{:?}", find_item("Borrower22", &brs, &Borrower::get_name));
//    //
//    //    let bk1 = Book::new("Title1", "Author1", None);
//    //    let bk2 = Book::new("Title2", "Author2", None);
//    //    let mut bks: Vec<&Book> = Vec::new();
//    //    bks.push(&bk1);
//    //    bks.push(&bk2);
//    //
//    //    println!("{:?}", find_item("Title1", &bks, &Book::get_title));
//    //    println!("{:?}", find_item("Title11", &bks, &Book::get_title));
//
//    let mut vikings = HashSet::new();
//
//    vikings.insert(Viking {
//        name: "Einar".to_string(),
//        power: 9,
//    });
//    vikings.insert(Viking {
//        name: "Einar".to_string(),
//        power: 9,
//    });
//    vikings.insert(Viking {
//        name: "Olaf".to_string(),
//        power: 4,
//    });
//    vikings.insert(Viking {
//        name: "Harald".to_string(),
//        power: 8,
//    });
//
//    // Use derived implementation to print the vikings.
//    for x in &vikings {
//        println!("{:?}", x);
//    }
//
//    println!(
//        "{:?}",
//        adder2(
//            Viking {
//                name: "HaraldXX".to_string(),
//                power: 8,
//            },
//            vikings
//        )
//    )
//}
//
//fn adder(x: Viking) -> HashSet<Viking> {
//    let mut vikings: HashSet<Viking> = HashSet::new();
//    vikings.insert(Viking {
//        name: "Eric".to_string(),
//        power: 9,
//    });
//    vikings.insert(x);
//    vikings
//}
//
//fn adder2(x: Viking, mut xs: HashSet<Viking>) -> HashSet<Viking> {
//    xs.insert(x);
//    xs
//}
//
//fn fun_test(value: i32, f: &Fn(i32) -> i32) -> i32 {
//    println!("{}", f(value));
//    value
//}
//
//fn times2(value: i32) -> i32 {
//    2 * value
//}
//
//fn is_br(br: &Borrower, f: &Fn(&Borrower) -> &str, target: &str) -> bool {
//    f(br) == target
//}
//
//pub fn find_borrower<'a>(
//    name: &str,
//    mut brs: &'a HashSet<Borrower>,
//    f: &Fn(&'a Borrower) -> &str,
//) -> Option<&'a Borrower> {
//    brs.iter().find(|&i| f(i) == name)
//}
//
//fn find_string<'a>(tgt: usize, coll: &'a [&'a str], f: &Fn(&str) -> usize) -> Option<&'a &'a str> {
//    coll.iter().find(|&&i| f(i) == tgt)
//}
//
//pub fn find_item<'a, T>(tgt: &str, coll: &'a Vec<&T>, f: &Fn(&'a T) -> &str) -> Option<&'a &'a T> {
//    coll.iter().find(|&&i| f(i) == tgt)
//}
//
////fn find_item<'a, T>(tgt: &str, coll: &'a mut Vec<T>, f: &Fn(&T) -> &str) -> Option<&'a T> {
////    coll.iter().find(|&i| f(i) == tgt)
////}
