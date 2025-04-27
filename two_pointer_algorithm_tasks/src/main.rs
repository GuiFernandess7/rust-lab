fn main() {
    let mut arr: Vec<usize> = vec![10, 5, 7, 3, 1, 4, 9, 8, 2, 6];

    println!("Array before sorting: {:?}", arr);
    let value_to_search = 7;
    let found = search_for_value(&mut arr, value_to_search);

    if found {
        println!("Value {} found in the array!", value_to_search);
    } else {
        println!("Value {} not found in the array.", value_to_search);
    }
}

fn search_for_value(arr: &mut Vec<usize>, value: usize) -> bool {
    // Binary Search
    arr.sort();
    if arr.is_empty() {
        return false;
    }

    let mut start = 0;
    let mut end = arr.len() - 1;

    while start <= end {
        let mid = (start + end) / 2;

        if value == arr[mid] {
            return true;
        }
        else if value > arr[mid] {
            start = mid + 1;
        }
        else if value < arr[mid] {
            if mid == 0 {
                break;
            }

            end = mid - 1;
        }
    }
    return false;
}

#[allow(dead_code)]
fn remove_value_from_array(arr: &mut Vec<usize>, value: usize){
    // Remove value from array - My Answer
    let mut right = arr.iter().len() - 1;
    let mut left = 0;

    while right >= left {
        if arr[left] == value {
            for i in (0..left).rev() {
                arr.swap(i, i + 1);
            }
            arr[0] = 0;
        }

        if arr[right] == value {
            for i in right..arr.iter().len() - 1 {
                arr.swap(i, i + 1);
            }

            if let Some(last) = arr.last_mut(){
                *last = 0;
            }
        }

        right -= 1;
        left += 1;
    }
}

#[allow(dead_code)]
fn remove_value_from_array2(arr: &mut Vec<usize>, value: usize) {
    // // Remove value from array - Best Answer
    let mut i = 0;

    for j in 0..arr.len() {
        if arr[j] != value {
            arr[i] = arr[j];
            i += 1;
        }
    }

    arr.truncate(i);
}

#[allow(dead_code)]
fn sum_values(a: i32, b: i32) -> i32 {
    return a + b;
}

#[allow(dead_code)]
fn is_odd(n: i32) -> bool {
    if n % 2 == 0 {
        return true
    }
    return false
}

#[allow(dead_code)]
fn two_pointer_test() -> usize {
    let end: i32 = 100;
    let mut mid_right: i32 = end / 2;
    let mut mid_left: i32 = end / 2 - 1;
    let mut file = File::create("data.txt").expect("Create failed");

    while mid_right - mid_left <= end {
        let result: i32 = sum_values(mid_right, mid_left);
        let mid_left_odd: bool = is_odd(mid_left);
        let mid_right_odd: bool = is_odd(mid_right);

        if mid_left_odd {
            mid_left -= 1;
        }

        if mid_right_odd {
            mid_right += 2;
        }

        print!("{} + {} = {} \n", mid_left, mid_right, result);
        file.write_all(format!("{} + {} = {}\n", mid_left, mid_right, result).as_bytes()).expect("Write failed");
    }
    return 0
}