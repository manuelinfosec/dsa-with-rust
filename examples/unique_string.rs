/// Problem:
/// Given a string, the task is to determine if all the characters in the string are unique or not.
/// Return true if all characters are unique, and false otherwise.
use std::collections::HashMap;

/// Checks if all characters in the given string are unique.
/// It uses a HashMap to store the occurrence of characters in the string.
/// If a character is encountered more than once, the function returns false; otherwise, it returns true.
///
/// Arguments:
/// - string: A String containing the input characters to check for uniqueness.
///
/// Returns:
/// - A boolean value: true if all characters in the string are unique, false otherwise.
///
/// Example:
/// ```
/// let string: &str = "hello";
/// assert_eq!(is_unique(string), false);
///
/// let string: &str = "world";
/// assert_eq!(is_unique(string), true);
///
/// let string: &str = "";
/// assert_eq!(is_unique(string), true);
/// ```
fn is_unique(string: &str) -> bool {
    // initialize a hash map for storing the occurrence of characters
    let mut string_map: HashMap<char, u32> = HashMap::new();

    // iterate through all the characters in the input string as type `char`
    for str in string.chars() {
        // check if the character exists in the map
        let exists: bool = string_map.contains_key(&str);

        // if it exists
        if exists {
            // "un-unique" string
            return false;
        }
        // if it does not exist
        else {
            // insert it into the map and continue iteration
            string_map.insert(str, 1);
        }
    }

    // end of loop indicates that no repititive characters was found in the hashmap
    true
}

fn main() {
    let string: &str = "chiemezie";
    let unique: bool = is_unique(string);

    if unique {
        println! {"{string} has no repititive characters."};
    } else {
        println! {"{string} has repititive characters."};
    }
}
