extern crate rand;

use rand::{thread_rng, Rng};

/**
 *  Implementing the Fisher-Yates shuffle algorithm,
 *   an algorithm for generating a random permutation of a sequence
 *      Together with Bogosort algorithm
 */

trait Swap {
    fn is_sorted(&self) -> bool;
}

impl Swap for Vec<usize> {
    fn is_sorted(&self) -> bool {
        // // iterate through the list and enumerate indexes
        // for i in 0..self.len() {
        //     let prev = self.get(i - 1).unwrap();
        // }

        for (idx, i) in self.iter().enumerate() {
            // if the previous index is greater than the current index
            if self[idx + 1] < *i {
                // array isn't sorted
                return false;
            }
        }

        true
    }
}

static mut COUNTER: i32 = 0;

fn shuffle(arr: &mut Vec<usize>) -> Vec<usize> {
    // derive vector size

    // iterate through the vector
    for idx in (0..arr.len()).rev() {
        let mut rng: rand::ThreadRng = thread_rng();

        // generate a random number between 0 and idx
        let random_idx: usize = rng.gen_range(0, idx + 1);

        // swap random_idx to current idx, without loosing random_idx's value
        arr.swap(idx, random_idx)
    }

    // return array
    arr.to_vec()
}

fn bogosort(mut arr: &mut Vec<usize>) -> Vec<usize> {
    while !arr.is_sorted() {
        shuffle(&mut arr);
        unsafe {
            COUNTER += 1;
            println!("{arr:?}, {COUNTER}");
        }
    }

    arr.to_owned()
}

fn main() {
    // let mut cards = Vector::new();
    let mut cards: Vec<usize> = vec![
        1, 5, 3, 4, 2, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    println!("Original array: {:?}", cards);

    // keep shuffling till cards is sorted
    let sorted_cards = bogosort(&mut cards);

    println!("Unsorted array: {cards:?}");
    println!("Shuffled array: {sorted_cards:?}");
}
