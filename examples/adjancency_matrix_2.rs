fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for numbers in matrix.iter() {
        println!("{numbers:?}");
    }
}

fn create_empty_matrix(n: i32) -> Vec<Vec<i32>> {
    // create an initial matrix
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    // loop through the initial matrix
    for a in 0..n {
        // push an empty matrix
        matrix.push(vec![]);

        for _b in 0..n {
            matrix[a as usize].push(0);
        }
    }
    return matrix;
}


fn main() {
    // let values: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut values: Vec<Vec<i32>> = create_empty_matrix(3);

    println!("Printing empty matrix: ");
    print_matrix(&values);


    for i in 0..3 {
        for j in 0..3 {
            let u = i + 1;
            let v = j + 1;

            let mut input = String::new();

            println!("Enter {u}-{v}: ");

            std::io::stdin().read_line(&mut input).unwrap();
            values[i][j] = input.trim().parse().unwrap();
        }
    }

    print_matrix(&values);
}