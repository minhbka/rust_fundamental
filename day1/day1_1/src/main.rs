
fn is_sub_array(array1: &[i32], array2: &[i32], n: i32, m: i32) -> bool {
    let mut i:i32 = 0;
    let mut j:i32 = 0;
    while i < n && j < m {
        if array1[i as usize] == array2[j as usize] {
            i += 1;
            j += 1;
            
            if j == m {
                return true;
            }
        }
        else{
            i = i - j + 1;
            j = 0;
        }
    }
    return false;
}

fn main() {
    
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let n = org_arr.len() as i32;
    let m = sub_arr.len() as i32;
    println!("is sub array: {}", is_sub_array(&org_arr, &sub_arr, n, m));
}
