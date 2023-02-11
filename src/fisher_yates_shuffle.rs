/*
    Implementing the Fisher-Yates shuffle algorithm,
    an algorithm for generating a random permutation of a sequence.
*/

extern crate rand;
use rand::Rng;

fn shuffle(mut arr: Vec<usize>) -> Vec<usize> {
    // derive vector size

    // iterate through the vector
    for idx in (0..arr.len()).rev() {
        // generate a random number between idx and 0
        let random_idx: usize = rand::thread_rng().gen_range(0, idx + 1);

        // swap random_idx to current idx, without loosing random_idx's value
        let temp: usize = arr[idx];
        arr[idx] = arr[random_idx];
        arr[random_idx] = temp;
    }

    // return array
    return arr;
}

fn main() {
    // let mut cards = Vector::new();
    let cards: Vec<usize> = vec![1, 2, 3, 4, 5];
    println!("Original array: {:?}", cards);

    // shuffle t
    let cards: Vec<usize> = shuffle(cards);
    println!("Randomized array: {cards:?}");
}
