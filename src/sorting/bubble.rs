pub fn bubble_sort(arr: &mut Vec<i32>) {
    return if arr.is_empty() {
    } else {
        let mut length = arr.len() - 1;
        let mut swapped;
        while length > 1 {
            swapped = false;
            for i in 0..length {
                if i < length && arr[i] > arr[i + 1] {
                    swapped = true;
                    arr.swap(i, i + 1);
                }
            }

            length -= 1;
            if !swapped {
                break;
            };
        }
    };
}

fn bubble_test() {
    let mut data = vec![-21, -43, -21, 9, -33, 5, 22, 83, 16];
    bubble_sort(&mut data);

    assert!(data, vec![-43, -33, -21, -21, 5, 9, 16, 22, 83]);

    data = vec![];
    bubble_sort(&mut data);

    assert!(data, vec![])
}
