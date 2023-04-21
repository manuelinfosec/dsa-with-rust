///
/// Creates n x n matrix filled with 0s
///
fn populate_matrix(&n: &i32) -> Vec<Vec<i32>> {
    // create an empty and mutable array
    let mut array: Vec<_> = Vec::new();

    // loop through the outer vector
    for a in 0..n {
        // push an empty vector before each iteration for population
        array.push(vec![]);

        // loop through inner vector
        for _ in 0..n {
            // push zero to inner vector n times
            array[a as usize].push(0);
        }
        
    }
    return array;
}

fn create_matrix(n: i32) -> Vec<Vec<i32>> {
    let matrix: Vec<Vec<i32>> = populate_matrix(&n);
    return matrix;
}

fn main() {
    let mat: Vec<Vec<i32>> = create_matrix(4);
    println!("{mat:?}");
}