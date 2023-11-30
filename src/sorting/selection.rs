use std::cmp::Ordering;

fn insertion_sort(arr: &mut Vec<i32>) {
    return if arr.is_empty() {
    } else {
        let mut outer = 0;

        let length = arr.len() - 1;
        while outer != length {
            let mut elem = outer;
            for i in (outer + 1)..=length {
                match arr[i].cmp(&arr[elem]) {
                    Ordering::Less => elem = i,
                    _ => (),
                }
            }
            match elem.cmp(&outer) {
                Ordering::Equal => (),
                _ => arr.swap(outer, elem),
            }
            outer += 1;
        }
    };
}
