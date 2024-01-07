// Using bitwise xor for swapping integers and booleans
fn xor_swap<T: Copy>(left: &mut T, right: &mut T) {
    *left = *left ^ *right;
    *right = *right ^ *left;
    *left = *left ^ *right;
}

fn is_power_of_two(num: i32) -> bool {
    (num & num - 1) == 0
}