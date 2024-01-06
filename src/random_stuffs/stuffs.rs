// Using bitwise xor for swapping integers and booleans
fn xor_swap<T: Copy>(left: &mut T, right: &mut T) {
    *left = *left ^ *right;
    *right = *right ^ *left;
    *left = *left ^ *right;
}
