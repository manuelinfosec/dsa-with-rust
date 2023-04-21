/*
   Fizz Buzz: Multiples of 3 and 5

   Fizz and Buzz refer to any number that's a multiple of 3 and 5 respectively. In other words, if a number is divisible
   by 3, it is substituted with fizz; if a number is divisible by 5, it is substituted with buzz. If a number is simultaneously
   a multiple of 3 AND 5, the number is replaced with "fizz buzz." In essence, it emulates the famous children game
   "fizz buzz".
*/

fn main() {
    // here 3 is fizz and 5 is buzz
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // to find all fizz and buzz, we must iterate through the vector and check which numbers are fizz
    // and which are buzz,

    for num in numbers {
        // check for 3 and 5 mod
        if num % 3 == 0 && num % 5 == 0 {
            println!("{num} -> fizz buzz")
        }
        // check for 3 mod
        else if num % 3 == 0 {
            println!("{num} -> Fizz");
        }
        // check for 5 mod
        else if num % 5 == 0 {
            println!("{num} -> Buzz");
        }
        // print number if neither 3 nor 5 is divisible
        else {
            println!("{}", num);
        }
    }
}
