use std::io;

trait Swap {
    fn is_sorted(&self) -> bool;
    fn swap(&mut self, i: usize, j: usize) -> Result<&mut Vec<i32>, io::Error>;
}

impl Swap for Vec<i32> {
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

    fn swap(&mut self, i: usize, j: usize) -> Result<&mut Vec<i32>, io::Error> {
        // let temp: i32;

        // check if i & j are the same values
        if i == j {
            // return an unchanged vector
            return Ok(self);
        }

        // return error if index is out of range
        if i > self.len() || j > self.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Index out of bounds",
            ));
        }

        // automatic way of swapping in a vector
        self.swap(i, j)?;

        // // save initial value to swap for later use
        // let temp = self[i];
        // // swap on both indices
        // self[i] = self[j];
        // // replace second index with temp
        // self[j] = temp;

        Ok(self)
    }
}

fn main() {
    let mut vector: Vec<i32> = Vec::from([1, 2, 3, 4, 5, 6, 7]);
    vector.swap(0, 4).expect("Out of range");
    let is_sorted: bool = vector.is_sorted();

    println!("{is_sorted}");
    println!("{vector:?}");
}
