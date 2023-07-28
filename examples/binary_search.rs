fn binary_search(array: Vec<i32>, value: i32) -> i32 {

    // initialize the lower point of the array
    let mut low: usize = 0;

    // initialize the highest point of the array
    let mut high: usize = array.len();

    // if the low is equal to or greater than high
    if low >= high {
        // not found
        return -1;
    }

    // while the lower index hasn't surpased the higher index
    while low < high {
        // determine mid-point: (low + high) / 2
        let mid: usize = (high + low) / 2; // confirm that is performs integer division
        println!("{mid}");

        // if the current mid point is the seeked value
        if array[mid] == value {
            // return the mid-point index
            return mid as i32;
        }

        // check if value is greater than or less than the mid point
        if value < array[mid] {
            // get rid of the upper half of the array by making the middle point become the new high
            high = mid;
        } else {
            // if the value is greater than the mid value, get rid of the lower half of the array
            // reassign low as the new mid point
            low = mid;
        }
    }

    // return not found
    -1
}

fn main() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let value: i32 = 2;
    let index: i32 = binary_search(values, value);

    println!("{value} is at index {index}");
}
