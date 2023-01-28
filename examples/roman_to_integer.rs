// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer.

// Example 1:

// Input: s = "III"
// Output: 3
// Explanation: III = 3.

use std::collections::HashMap;
use std::iter::Peekable;

extern crate lazy_static;

// `lazy_static` is used to declare the mapping as a static variable because it
// allows for initializion lazily on its first use. This means that the HashMap
// will only be used once, the first time it it accessed and subsequent accesses
// will reuse the exisiting HashMap.
lazy_static::lazy_static! {
    static ref MAPPING: HashMap<char, i32> = {
        let mut map = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);
        map
    };
}

fn roman_to_int(s: String) -> i32 {
    // Create a character iterator over the input string `s` which allows us to iterate through its characters.
    // The `peekable()` method is used to create a peekable iterator, which allows us to look ahead at the next element
    // of the iterator without consuming it.
    let mut symbols: Peekable<std::str::Chars<'_>> = s.chars().peekable();

    // Initialize the variable `result` to store the final integer result of converting the Roman numeral to an integer.
    let mut result: i32 = 0;

    // Iterate through each character of the input string `s`.
    while let Some(current_char) = symbols.next() {
        // Look up the integer value associated with the current Roman numeral character (`current_char`) in the `MAPPING` HashMap.
        // If the character is not found in the HashMap, default to 0.
        let current: i32 = *MAPPING.get(&current_char).unwrap_or(&0);

        // Check if there is a next character available in the iterator.
        // The `peek()` method is used to peek at the next character without consuming it.
        // The `and_then()` method is used to perform an operation if there is a next character,
        // otherwise it returns `None`.
        if let Some(&next) = symbols.peek().and_then(|&c| MAPPING.get(&c)) {
            // If the value of the next character is greater than the value of the current character,
            // subtract the value of the current character from the value of the next character
            // and add the result to the `result`.
            if next > current {
                result += next - current;
                symbols.next(); // Move to the next character by consuming it
            } else {
                // If the value of the next character is not greater than the value of the current character,
                // simply add the value of the current character to the `result`.
                result += current;
            }
        } else {
            // If there is no next character available in the iterator,
            // add the value of the current character to the `result`.
            result += current;
        }
    }

    // Return the final integer result representing the converted Roman numeral.
    result
}

fn main() {
    let symbol: String = String::from("MCMXCIV");
    println!("{0:?}", roman_to_int(symbol));
}
