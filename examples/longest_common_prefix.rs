/*
LeetCode: Longest Common Prefix
Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".


Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"
Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
*/

// Approach
// iterate through all words in vector
// compare all first characters and append to `answer`.
// return `answer` when a character mismatch is met

pub fn longest_common_prefix1(strs: Vec<String>) -> String {
    // check if input vector is empty
    if strs.is_empty() {
        return String::new();
    }

    // placeholder for return value
    let mut answer: String = String::new();
    // counter for positional index
    let mut pos: usize = 0;

    // using tags for loops
    'outer: loop {
        // collect the current character in the first word
        let current_char = match strs[0].chars().nth(pos) {
            // if a value is found, assign
            Some(c) => c,
            // if no value is found, break the loop
            None => break 'outer,
        };

        // check if second character has a value
        if strs.get(1).is_none() {
            return strs[0].clone();
        }

        // loop through all other strings in vector, except the first
        for s in strs.iter().skip(1) {
            // match the current character based on positional index
            match s.chars().nth(pos) {
                // if a value is found
                Some(next_char) => {
                    // check if it matches the current character in the first word
                    if current_char != next_char {
                        // break loop
                        break 'outer;
                    }
                }
                // if no character is found in the string
                None => break 'outer,
            }
        }

        // push matched characters to placeholder
        answer.push(current_char);
        // increase positional index by 1
        pos += 1;
    }
    // return placeholder as answer
    answer
}

fn longest_common_prefix(input: Vec<String>) -> String {
    input
        // convert input to iterator
        .into_iter()
        // reduce iterator to accumulator and current element
        .reduce(|acc: String, curr: String| {
            // convert accumulator string to character
            acc.chars()
                // combine charactered accumulator and charactered current element to a tuple
                .zip(curr.chars()) // tuple returned
                // iterate through both elements of tuple ending when both don't equate
                .take_while(|(a, c)| a == c) // tuple returned
                // map the tuples and return any of its sides
                .map(|(_, c)| c)
                // convert iterator back to string
                .collect::<String>()
        })
        // called on `reduce` there is at least one item in the vector, so this is safe
        .unwrap()
}

fn main() {
    let _strs: Vec<String> = vec![
        "andre".to_string(),
        "antelope".to_string(),
        "anty".to_string(),
    ];

    let strs: Vec<String> = vec!["a".to_string()];
    let longest: String = longest_common_prefix(strs);
    println!("{longest}");
}
