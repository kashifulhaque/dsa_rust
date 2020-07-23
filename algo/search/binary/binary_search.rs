fn main() {
    let mut arr: Vec<i32> = Vec::new();
    arr.push(3);
    arr.push(6);
    arr.push(8);
    arr.push(12);
    arr.push(14);
    arr.push(17);
    arr.push(25);
    arr.push(29);
    arr.push(31);
    arr.push(36);
    arr.push(42);
    arr.push(47);
    arr.push(53);
    arr.push(55);
    arr.push(64);

    println!("Result: {}", r_search(&arr, 31));
}

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
            helper(arr, item, l, mid - 1)
        } else {
            helper(arr, item, mid + 1, h)
        }
    }
}
