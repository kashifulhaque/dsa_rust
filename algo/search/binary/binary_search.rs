// Binary Search using Iteration
fn search(arr: &Vec<i32>, item: i32) -> i32 {
    if arr.len() == 0 {
        return -1;
    }

    let mut l: usize = 0;
    let mut h: usize = arr.len() - 1;
    while l <= h {
        let mid: usize = (l + h) / 2;

        if arr[mid] == item {
            return mid as i32;
        } else if item < arr[mid] {
            h = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    -1
}

// Binary Search using recursion
fn r_search(arr: &Vec<i32>, item: i32) -> i32 {
    helper(&arr, item, 0, arr.len() - 1)
}

fn helper(arr: &Vec<i32>, item: i32, l: usize, h: usize) -> i32 {
    if l == h {
        if arr[l] == item {
            l as i32
        } else {
            -1
        }
    } else {
        let mid: usize = (l + h) / 2;
        if item == arr[mid] {
            return mid as i32;
        }

        if item < arr[mid] {
            helper(&arr, item, l, mid - 1)
        } else {
            helper(&arr, item, mid + 1, h)
        }
    }
}
