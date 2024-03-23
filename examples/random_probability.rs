/// Given a function with items, their probabilities of occurrence,
/// and number of items to return (k), return a random sample from
/// the items based on the odds assigned to each.
extern crate rand;

use rand::{random, Rng};

fn random_draw<T>(items: Vec<T>, weights: Vec<f32>, _k: i32) -> T
where
    T: std::fmt::Debug + std::fmt::Display,
{
    // Generate a random number between 0 and 1
    let mut random_number: f32 = rand::thread_rng().gen_range::<f32>(0.0, 1.0);

    items
        // convert into an iterable
        .into_iter()
        // zip both vectors to make an iterable of tuple
        .zip(weights)
        // iteratte until the random number is less than zero
        .take_while(|(_, b)| {
            // subtract the probabilties from the random number
            random_number -= b;
            random_number > 0 as f32
        })
        // collect the last remaining item if it exists
        .last()
        // will always exist
        .unwrap()
        // collect item from the tuple
        .0
}

fn main() {
    let random_choice: i32 = random_draw(vec![1, 2, 3, 4, 5], vec![0.1, 0.2, 0.3, 0.2, 0.2], 3);
    println!("{random_choice}");
}
