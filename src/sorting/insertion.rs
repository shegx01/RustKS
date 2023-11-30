use std::cmp::Ordering;

fn insertion_sort(vec: &mut Vec<i32>) {
    return if vec.is_empty() {
    } else {
        let mut last_sorted_idx = 1;
        while last_sorted_idx != vec.len() {
            let current_item = vec[last_sorted_idx];
            for i in (0..last_sorted_idx).rev() {
                match vec[i].cmp(&current_item) {
                    Ordering::Greater => vec.swap(i, i + 1),
                    _ => {}
                }
            }

            last_sorted_idx += 1;
        }
    };
}
