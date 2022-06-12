fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    let result = is_subarr(&sub_arr, &org_arr);

    println!("result: {:#?}", result);
    
}
fn is_subarr(array: &[i32], other_array: &[i32]) -> bool {
    if array.len() == 0 {
        return true;
    }

    for frame in other_array.windows(array.len()) {
        if frame == array {
            return true;
        }
    }

    return false;
}