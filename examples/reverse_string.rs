/**
 * String Reverse
 *
 * Given a string, reverse it.
 *
 */
// import for explicit annotation
use std::str::Chars;

// create a new trait for String
trait Reverse {
    fn reverse(self) -> String;
}

//
impl Reverse for String {
    fn reverse(self) -> String {
        // initialize a new string
        let mut new_string = String::new();

        // convert string to iterable
        let mut iter_string: Chars = self.chars();

        // if a value exists from behind
        while let Some(c) = iter_string.next_back() {
            // collect c and push to `new_string`
            String::push(&mut new_string, c);
        }

        // return the reversed string
        return new_string;
    }
}

fn main() {
    // convert `'static &str` to String and call the reverse method
    let reversed_string: String = "racecar".to_string().reverse();

    // print reversed string
    println!("{reversed_string}");
}
