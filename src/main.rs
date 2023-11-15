use std::collections::HashMap;

fn main() {
    use std::collections::HashMap;
    // let arr = [0u8; 20];
    let arr = vec![0u8; 20];
    let res: Vec<u8> = arr.iter().map(|a| a + 2).collect();

    // get the ith element
    // though reference
    // let hd = &arr[0];
    // copy the index value NOTE!: Copy trait must be implemented
    let hd = arr[0];

    let data: Vec<&str> = vec!["Hello", "World"];

    // reference the head
    let hd = data[0];

    // The Slice f.first returns an Option<&T>
    // .unwrap() is needed to extract the actual value if you are sure it wont panic

    // cloning the
    let hd = data[0].clone();

    //  getting part of the array
    let res2 = res[2..=5].to_vec();
    let res3 = res.clone().get_mut(2).unwrap();

    let mut months = vec!["July", "August", "September", "November", "October"];
    months.swap(3, 4);

    let slice1 = [1, 2, 3, 4];
    let slice2 = [1, 2, 3, 4];
    println!("{:?}", hd);
    println!("{:?}", res2);
    println!("{:?}", arr);
    println!("{:?}", &res[0]);
    println!("Months are {:?}", months);

    let mut map = HashMap::new();

    map.insert("u", 21);

    map.insert("w", 16);

    println!("Map contains: {:?}", map);

    let entry = map.entry("x").or_insert(90);
    println!("Returned entry is: {:?}", entry);

    let equal = if slice1 == slice2 { "Yes" } else { "No" };
    println!("Are they equal? {}", equal)
}
