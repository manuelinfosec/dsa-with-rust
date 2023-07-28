fn binary_search_iter(array: Vec<i32>, value: i32) -> i32 {
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

fn binary_search_recr(array: Vec<i32>, value: i32, low: usize, high: usize) -> i32 {
    // compute the mid-point of the array
    let mid: usize = (low + high) / 2;

    // for a base case, check if the low is equal of less than the high
    if low >= high {
        // return not found
        return -1;
    }
    println!("{low} {high}");

    // check if the mid value is the current value
    if array[mid as usize] == value {
        // return found index
        return mid as i32;
    }

    // if the mid-value is lower than the specified value
    if array[mid] < value {
        // re-call the current function making mid-point the new low index
        return binary_search_recr(array, value, mid, high);
    // if the mid-value is higher than the specified value
    } else {
        // re-call the currenct function making the mid-point the new high index
        return binary_search_recr(array, value, low, mid);
    }
}

fn main() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let value: i32 = 0;
    // this is done before any of the functions take ownership of the values
    let values_length: usize = values.len();

    // let index: i32 = binary_search_iter(values, value);
    let index: i32 = binary_search_recr(values, value, 0, values_length);

    assert_eq!(index, 1);
}
