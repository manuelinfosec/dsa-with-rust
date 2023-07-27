fn is_palindrome(x: i32) -> bool {
    // check if x is less than 0
    if x < 0 {
        // palindromic numbers cannot be less than 0
        // this can also be done by accepting u32 as type
        return false;
    }

    // convert x to an iterable vector
    let iter_string: Vec<char> = x.to_string().chars().collect();

    // index length of string to unsigned 32-bit integer
    let length: i32 = iter_string.len() as i32;

    // index the left pointer at the beginning of the string
    let mut left_pointer: usize = 0;
    // index the right pointer as the length of the string
    let mut right_pointer: usize = (length - 1) as usize;

    // check if the left pointer cross each other
    while left_pointer <= right_pointer {
        
        // check if values at left and right pointers don't match
        if iter_string[left_pointer] != iter_string[right_pointer] {
            // if the values at any point in the string does not match, then it isn't a palindrom
            return false;
        }
        
        // change position of the left pointer to character right
        left_pointer += 1;
        
        // change position of the right pointer to a character left
        right_pointer -= 1;
    }

    // if after looping through and none of the base conditions are met, then it's a valid palindrome
    return true;
}

fn main() {
    // input
    let x: i32 = 33133;

    // print and check if palindrome
    println!("{x} is a palindrome: {:?}", is_palindrome(x));
}
