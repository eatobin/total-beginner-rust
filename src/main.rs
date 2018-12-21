#[cfg(test)]
mod borrower;

fn main() {
    let mut noodles: String = "noodles".to_string();
    let oodles: &mut str = &mut noodles[1..];
    oodles.make_ascii_uppercase();
    println!("{}", oodles);
}
