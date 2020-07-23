// Binary Search using Iteration
fn search(arr: &Vec<i32>, item: i32) -> i32 {
    if arr.len() == 0 {
        -1
    }

    let mut l: usize = 0;
    let mut h: usize = arr.len() - 1;
    while l <= h {
        let mid: usize = (l + h) / 2;

        if arr[mid] == item {
            mid as i32
        } else if item < arr[mid] {
            h = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    -1
}
