/*
 * Two Sums
 *
 * Given an array of integers `nums` and an integer `target`,
 * return indices of the two numbers such that they add up to target.
 *
*/

use std::collections::{HashMap, HashSet};

// less efficient
// fn two_sums(nums: Vec<i32>, target: i32) -> (i32, i32) {
//     let mut mapping: HashMap<i32, i32> = HashMap::new();
//
//     let mut retval: i32 = 0;
//     let mut retidx: i32 = 0;
//
//     for (idx, val) in nums.iter().enumerate() {
//         let eval = target - val;
//
//         if mapping.contains_key(&eval) {
//             retval = *mapping.get(&eval).unwrap();
//             retidx = idx.try_into().unwrap();
//         } else {
//             mapping.insert(*val, idx.try_into().unwrap());
//         }
//     }
//     return (retval, retidx);
// }

// more efficient
fn two_sums(nums: Vec<i32>, target: i32) -> (i32, i32) {
    // use a hashset for the sake of unique values
    let mut mapping: HashSet<i32> = HashSet::<i32>::new();

    let mut retval: i32 = 0;
    let mut retidx: i32 = 0;

    // use of less memory space for storing index and value
    for idx in 0..nums.len() {
        let val: i32 = nums[idx];
        let eval: i32 = target - val;

        // reduce number of hash lookups
        if let Some(pos) = mapping.get(&eval) {
            retval = *pos;
            retidx = idx as i32;
        } else {
            mapping.insert(val);
        }
    }

    // make code more rusty
    (retval, retidx)
}

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 17;

    let solution: (i32, i32) = two_sums(nums, target);

    println!("{solution:?}");
}

// declare nums and target
// loop through nums and index values to a mapping {number: index}
// if (target - current) value is in the mapping, return  current index...
// ...and value of the other number in mapping
