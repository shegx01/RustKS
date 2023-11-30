use std::cmp::Ordering;
fn merge_sort<T: Clone + std::cmp::Ord + Copy>(vec: Vec<T>) -> Vec<T> {
    if vec.len() <= 1 {
        return vec;
    }
    let mid = vec.len() / 2;
    let sorted_left = merge_sort(vec[..mid].to_vec());
    let sorted_right = merge_sort(vec[mid..].to_vec());
    merge(&sorted_left, &sorted_right)
}

fn merge<T>(vec1: &Vec<T>, vec2: &Vec<T>) -> Vec<T>
where
    T: std::cmp::Ord + Copy,
{
    let mut data = Vec::with_capacity(vec1.len());
    let mut idx_one: usize = 0;
    let mut idx_two = 0;

    while idx_one < vec1.len() && idx_two < vec2.len() {
        match vec1[idx_one].cmp(&vec2[idx_two]) {
            Ordering::Less => {
                data.push(vec1[idx_one]);
                idx_one += 1;
            }
            _ => {
                data.push(vec2[idx_two]);
                idx_two += 1;
            }
        }
    }

    if idx_one < vec1.len() {
        data.extend(&vec1[idx_one..])
    };
    if idx_two < vec2.len() {
        data.extend(&vec2[idx_two..])
    };

    data
}
