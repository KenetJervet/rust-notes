pub fn vectors() {
    let vec: Vec<i32> = Vec::new();
    let vec2 = vec![1, 2, 3, 4, 5];

    // mutate a vector
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("vec is {:?}", vec);

    invalid_references();
    using_enums_to_store_multiple_types();
}

fn invalid_references() {
    let mut v = vec![1, 2, 3];
    let first_element = &v[0];
    // v.clear();  // Err: cannot borrow as mutable and immutable at the same time
    println!("First element is {}", first_element);
}

fn using_enums_to_store_multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Row is {:?}", row);
}
