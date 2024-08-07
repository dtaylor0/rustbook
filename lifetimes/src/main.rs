use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    // Single lifetime used to indicate the shortest lifetime applies to output
    T: Display, // Generic must implement display to be printed with {} formatting
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "Hello world!";
    let s2 = "This is a very long one.";
    let ann = 33.3323;

    let longest = longest_with_an_announcement(s1, s2, ann);
    println!("{longest}");
}
